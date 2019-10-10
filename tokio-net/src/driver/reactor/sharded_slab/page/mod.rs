use super::{Pack, ScheduledIo, INITIAL_PAGE_SIZE, WIDTH};
use crate::sync::{
    atomic::{AtomicUsize, Ordering},
    CausalCell,
};

pub(crate) mod slot;
use self::slot::Slot;
use std::fmt;

/// A page address encodes the location of a slot within a shard (the page
/// number and offset within that page) as a single linear value.
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, PartialOrd, Ord)]
pub(crate) struct Addr {
    addr: usize,
}

impl Addr {
    const NULL: usize = Self::BITS + 1;
    const INDEX_SHIFT: usize = INITIAL_PAGE_SIZE.trailing_zeros() as usize + 1;

    pub(crate) fn index(self) -> usize {
        // Since every page is twice as large as the previous page, and all page sizes
        // are powers of two, we can determine the page index that contains a given
        // address by shifting the address down by the smallest page size and
        // looking at how many twos places necessary to represent that number,
        // telling us what power of two page size it fits inside of. We can
        // determine the number of twos places by counting the number of leading
        // zeros (unused twos places) in the number's binary representation, and
        // subtracting that count from the total number of bits in a word.
        WIDTH - ((self.addr + INITIAL_PAGE_SIZE) >> Self::INDEX_SHIFT).leading_zeros() as usize
    }

    pub(crate) fn offset(self) -> usize {
        self.addr
    }
}

pub(super) fn size(n: usize) -> usize {
    INITIAL_PAGE_SIZE * 2usize.pow(n as _)
}

impl Pack for Addr {
    const LEN: usize = super::MAX_PAGES + Self::INDEX_SHIFT;

    type Prev = ();

    fn as_usize(&self) -> usize {
        self.addr
    }

    fn from_usize(addr: usize) -> Self {
        debug_assert!(addr <= Self::BITS);
        Self { addr }
    }
}

pub(in crate::driver) type Iter<'a> =
    std::iter::FilterMap<std::slice::Iter<'a, Slot>, fn(&'a Slot) -> Option<&'a ScheduledIo>>;

pub(crate) struct Local {
    head: CausalCell<usize>,
}

pub(crate) struct Shared {
    remote_head: AtomicUsize,
    size: usize,
    prev_sz: usize,
    slab: CausalCell<Option<Box<[Slot]>>>,
}

impl Local {
    pub(crate) fn new() -> Self {
        Self {
            head: CausalCell::new(0),
        }
    }

    #[inline(always)]
    fn head(&self) -> usize {
        self.head.with(|head| unsafe { *head })
    }

    #[inline(always)]
    fn set_head(&self, new_head: usize) {
        self.head.with_mut(|head| unsafe {
            *head = new_head;
        })
    }
}

impl Shared {
    const NULL: usize = Addr::NULL;

    pub(crate) fn new(size: usize, prev_sz: usize) -> Self {
        Self {
            prev_sz,
            size,
            remote_head: AtomicUsize::new(Self::NULL),
            slab: CausalCell::new(None),
        }
    }

    #[cold]
    fn fill(&self) {
        #[cfg(test)]
        println!("-> alloc new page ({})", self.size);

        debug_assert!(self.slab.with(|s| unsafe { (*s).is_none() }));

        let mut slab = Vec::with_capacity(self.size);
        slab.extend((1..self.size).map(Slot::new));
        slab.push(Slot::new(Self::NULL));
        self.slab.with_mut(|s| {
            // this mut access is safe — it only occurs to initially
            // allocate the page, which only happens on this thread; if the
            // page has not yet been allocated, other threads will not try
            // to access it yet.
            unsafe {
                *s = Some(slab.into_boxed_slice());
            }
        });
    }

    #[inline]
    pub(crate) fn insert(&self, local: &Local, aba_guard: usize) -> Option<usize> {
        let head = local.head();
        #[cfg(test)]
        println!("-> local head {:?}", head);

        // are there any items on the local free list? (fast path)
        let head = if head < self.size {
            head
        } else {
            // if the local free list is empty, pop all the items on the remote
            // free list onto the local free list.
            self.remote_head.swap(Self::NULL, Ordering::Acquire)
        };

        // if the head is still null, both the local and remote free lists are
        // empty --- we can't fit any more items on this page.
        if head == Self::NULL {
            #[cfg(test)]
            println!("-> NULL! {:?}", head);
            return None;
        }

        // do we need to allocate storage for this page?
        let page_exists = self.slab.with(|s| unsafe { (*s).is_none() });
        if page_exists {
            self.fill();
        }

        self.slab.with(|slab| {
            let slab = unsafe { &*(slab) }
                .as_ref()
                .expect("page must have been allocated to insert!");
            let slot = &slab[head];
            slot.insert(aba_guard);
            local.set_head(slot.next());
        });

        let index = head + self.prev_sz;
        #[cfg(test)]
        println!("insert at offset: {}", index);
        Some(index)
    }

    #[inline]
    pub(in crate::driver) fn get(&self, addr: Addr) -> Option<&ScheduledIo> {
        let page_offset = addr.offset() - self.prev_sz;
        #[cfg(test)]
        println!("-> offset {:?}", page_offset);

        self.slab
            .with(|slab| unsafe { &*slab }.as_ref()?.get(page_offset)?.value())
    }

    pub(crate) fn remove_local(&self, local: &Local, addr: Addr) {
        let offset = addr.offset() - self.prev_sz;

        #[cfg(test)]
        println!("-> offset {:?}", offset);

        self.slab.with(|slab| {
            let slot =
                if let Some(slot) = unsafe { &*slab }.as_ref().and_then(|slab| slab.get(offset)) {
                    slot
                } else {
                    return;
                };
            if slot.reset() {
                slot.set_next(local.head());
                local.set_head(offset);
            }
        })
    }

    pub(crate) fn remove_remote(&self, addr: Addr) {
        let offset = addr.offset() - self.prev_sz;

        #[cfg(test)]
        println!("-> offset {:?}", offset);

        self.slab.with(|slab| {
            let slot =
                if let Some(slot) = unsafe { &*slab }.as_ref().and_then(|slab| slab.get(offset)) {
                    slot
                } else {
                    return;
                };
            if !slot.reset() {
                return;
            }

            let mut next = self.remote_head.load(Ordering::Relaxed);
            loop {
                #[cfg(test)]
                println!("-> next={:?}", next);

                slot.set_next(next);

                let res = self.remote_head.compare_exchange(
                    next,
                    offset,
                    Ordering::AcqRel,
                    Ordering::Acquire,
                );
                match res {
                    Ok(_) => return,
                    Err(actual) => next = actual,
                }
            }
        })
    }

    pub(in crate::driver) fn iter(&self) -> Option<Iter<'_>> {
        let slab = self.slab.with(|slab| unsafe { (&*slab).as_ref() });
        slab.map(|slab| slab.iter().filter_map(Slot::value as fn(_) -> _))
    }
}

impl fmt::Debug for Local {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.head.with(|head| {
            let head = unsafe { *head };
            f.debug_struct("Local")
                .field("head", &format_args!("{:#0x}", head))
                .finish()
        })
    }
}

impl fmt::Debug for Shared {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Shared")
            .field(
                "remote_head",
                &format_args!("{:#0x}", &self.remote_head.load(Ordering::Relaxed)),
            )
            .field("prev_sz", &self.prev_sz)
            .field("size", &self.size)
            // .field("slab", &self.slab)
            .finish()
    }
}

impl fmt::Debug for Addr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Addr")
            .field("addr", &format_args!("{:#0x}", &self.addr))
            .field("index", &self.index())
            .field("offset", &self.offset())
            .finish()
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use proptest::prelude::*;

    proptest! {
        #[test]
        fn addr_roundtrips(pidx in 0usize..Addr::BITS) {
            let addr = Addr::from_usize(pidx);
            let packed = addr.pack(0);
            assert_eq!(addr, Addr::from_packed(packed));
        }
    }
}
