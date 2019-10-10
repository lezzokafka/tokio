//! A lock-free concurrent slab.
mod iter;
mod pack;
mod page;
mod slab;
mod tid;

use super::ScheduledIo;
use pack::{Pack, WIDTH};
use slab::Shard;
pub(crate) use slab::{Slab, MAX_SOURCES, TOKEN_SHIFT};
use tid::Tid;

#[cfg(target_pointer_width = "64")]
const MAX_THREADS: usize = 4096;
#[cfg(target_pointer_width = "32")]
const MAX_THREADS: usize = 2048;
const INITIAL_PAGE_SIZE: usize = 32;
const MAX_PAGES: usize = WIDTH / 4;

#[cfg(test)]
mod tests;
