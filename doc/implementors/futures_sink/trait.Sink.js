(function() {var implementors = {};
implementors["tokio_codec"] = [{text:"impl&lt;T, I, U&gt; <a class=\"trait\" href=\"https://rust-lang-nursery.github.io/futures-api-docs/0.3.0-alpha.19/futures_sink/futures_sink/trait.Sink.html\" title=\"trait futures_sink::Sink\">Sink</a>&lt;I&gt; for <a class=\"struct\" href=\"tokio_codec/struct.Framed.html\" title=\"struct tokio_codec::Framed\">Framed</a>&lt;T, U&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;T: <a class=\"trait\" href=\"tokio_io/async_write/trait.AsyncWrite.html\" title=\"trait tokio_io::async_write::AsyncWrite\">AsyncWrite</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/marker/trait.Unpin.html\" title=\"trait core::marker::Unpin\">Unpin</a>,<br>&nbsp;&nbsp;&nbsp;&nbsp;U: <a class=\"trait\" href=\"tokio_codec/trait.Encoder.html\" title=\"trait tokio_codec::Encoder\">Encoder</a>&lt;Item = I&gt; + <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/marker/trait.Unpin.html\" title=\"trait core::marker::Unpin\">Unpin</a>,<br>&nbsp;&nbsp;&nbsp;&nbsp;U::<a class=\"type\" href=\"tokio_codec/trait.Encoder.html#associatedtype.Error\" title=\"type tokio_codec::Encoder::Error\">Error</a>: <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;<a class=\"struct\" href=\"https://doc.rust-lang.org/nightly/std/io/error/struct.Error.html\" title=\"struct std::io::error::Error\">Error</a>&gt;,&nbsp;</span>",synthetic:false,types:["tokio_codec::framed::Framed"]},{text:"impl&lt;T, I, D&gt; <a class=\"trait\" href=\"https://rust-lang-nursery.github.io/futures-api-docs/0.3.0-alpha.19/futures_sink/futures_sink/trait.Sink.html\" title=\"trait futures_sink::Sink\">Sink</a>&lt;I&gt; for <a class=\"struct\" href=\"tokio_codec/struct.FramedRead.html\" title=\"struct tokio_codec::FramedRead\">FramedRead</a>&lt;T, D&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;T: <a class=\"trait\" href=\"https://rust-lang-nursery.github.io/futures-api-docs/0.3.0-alpha.19/futures_sink/futures_sink/trait.Sink.html\" title=\"trait futures_sink::Sink\">Sink</a>&lt;I&gt; + <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/marker/trait.Unpin.html\" title=\"trait core::marker::Unpin\">Unpin</a>,<br>&nbsp;&nbsp;&nbsp;&nbsp;D: <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/marker/trait.Unpin.html\" title=\"trait core::marker::Unpin\">Unpin</a>,&nbsp;</span>",synthetic:false,types:["tokio_codec::framed_read::FramedRead"]},{text:"impl&lt;T, I, E&gt; <a class=\"trait\" href=\"https://rust-lang-nursery.github.io/futures-api-docs/0.3.0-alpha.19/futures_sink/futures_sink/trait.Sink.html\" title=\"trait futures_sink::Sink\">Sink</a>&lt;I&gt; for <a class=\"struct\" href=\"tokio_codec/struct.FramedWrite.html\" title=\"struct tokio_codec::FramedWrite\">FramedWrite</a>&lt;T, E&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;T: <a class=\"trait\" href=\"tokio_io/async_write/trait.AsyncWrite.html\" title=\"trait tokio_io::async_write::AsyncWrite\">AsyncWrite</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/marker/trait.Unpin.html\" title=\"trait core::marker::Unpin\">Unpin</a>,<br>&nbsp;&nbsp;&nbsp;&nbsp;E: <a class=\"trait\" href=\"tokio_codec/trait.Encoder.html\" title=\"trait tokio_codec::Encoder\">Encoder</a>&lt;Item = I&gt; + <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/marker/trait.Unpin.html\" title=\"trait core::marker::Unpin\">Unpin</a>,<br>&nbsp;&nbsp;&nbsp;&nbsp;E::<a class=\"type\" href=\"tokio_codec/trait.Encoder.html#associatedtype.Error\" title=\"type tokio_codec::Encoder::Error\">Error</a>: <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;<a class=\"struct\" href=\"https://doc.rust-lang.org/nightly/std/io/error/struct.Error.html\" title=\"struct std::io::error::Error\">Error</a>&gt;,&nbsp;</span>",synthetic:false,types:["tokio_codec::framed_write::FramedWrite"]},];
implementors["tokio_net"] = [{text:"impl&lt;C:&nbsp;<a class=\"trait\" href=\"tokio_codec/encoder/trait.Encoder.html\" title=\"trait tokio_codec::encoder::Encoder\">Encoder</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/marker/trait.Unpin.html\" title=\"trait core::marker::Unpin\">Unpin</a>&gt; <a class=\"trait\" href=\"https://rust-lang-nursery.github.io/futures-api-docs/0.3.0-alpha.19/futures_sink/futures_sink/trait.Sink.html\" title=\"trait futures_sink::Sink\">Sink</a>&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.tuple.html\">(</a>&lt;C as <a class=\"trait\" href=\"tokio_codec/encoder/trait.Encoder.html\" title=\"trait tokio_codec::encoder::Encoder\">Encoder</a>&gt;::<a class=\"type\" href=\"tokio_codec/encoder/trait.Encoder.html#associatedtype.Item\" title=\"type tokio_codec::encoder::Encoder::Item\">Item</a>, <a class=\"enum\" href=\"https://doc.rust-lang.org/nightly/std/net/addr/enum.SocketAddr.html\" title=\"enum std::net::addr::SocketAddr\">SocketAddr</a><a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.tuple.html\">)</a>&gt; for <a class=\"struct\" href=\"tokio_net/udp/struct.UdpFramed.html\" title=\"struct tokio_net::udp::UdpFramed\">UdpFramed</a>&lt;C&gt;",synthetic:false,types:["tokio_net::udp::frame::UdpFramed"]},];
implementors["tokio_sync"] = [{text:"impl&lt;T&gt; <a class=\"trait\" href=\"https://rust-lang-nursery.github.io/futures-api-docs/0.3.0-alpha.19/futures_sink/futures_sink/trait.Sink.html\" title=\"trait futures_sink::Sink\">Sink</a>&lt;T&gt; for <a class=\"struct\" href=\"tokio_sync/mpsc/struct.Sender.html\" title=\"struct tokio_sync::mpsc::Sender\">Sender</a>&lt;T&gt;",synthetic:false,types:["tokio_sync::mpsc::bounded::Sender"]},{text:"impl&lt;T&gt; <a class=\"trait\" href=\"https://rust-lang-nursery.github.io/futures-api-docs/0.3.0-alpha.19/futures_sink/futures_sink/trait.Sink.html\" title=\"trait futures_sink::Sink\">Sink</a>&lt;T&gt; for <a class=\"struct\" href=\"tokio_sync/mpsc/struct.UnboundedSender.html\" title=\"struct tokio_sync::mpsc::UnboundedSender\">UnboundedSender</a>&lt;T&gt;",synthetic:false,types:["tokio_sync::mpsc::unbounded::UnboundedSender"]},{text:"impl&lt;T&gt; <a class=\"trait\" href=\"https://rust-lang-nursery.github.io/futures-api-docs/0.3.0-alpha.19/futures_sink/futures_sink/trait.Sink.html\" title=\"trait futures_sink::Sink\">Sink</a>&lt;T&gt; for <a class=\"struct\" href=\"tokio_sync/watch/struct.Sender.html\" title=\"struct tokio_sync::watch::Sender\">Sender</a>&lt;T&gt;",synthetic:false,types:["tokio_sync::watch::Sender"]},];

            if (window.register_implementors) {
                window.register_implementors(implementors);
            } else {
                window.pending_implementors = implementors;
            }
        })()