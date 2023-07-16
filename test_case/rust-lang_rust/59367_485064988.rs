shell
 Documenting tokio v0.1.18
 Documenting tokio v0.1.18 (/data/doc/code/tokio/tokio)
 Documenting serde_cbor v0.9.0
error: `[throttle]` cannot be resolved, ignoring it...
  --> /data/doc/code/tokio/tokio/src/util/stream.rs:13:48
   |
13 | /// Currently, there are only [`timeout`] and [`throttle`] functions, but
   |                                                ^^^^^^^^^^ cannot be resolved, ignoring
   |
note: lint level defined here
  --> /data/doc/code/tokio/tokio/src/lib.rs:2:23
   |
2  | #![deny(missing_docs, warnings, missing_debug_implementations)]
   |                       ^^^^^^^^
   = note: #[deny(intra_doc_link_resolution_failure)] implied by #[deny(warnings)]
   = help: to escape `[` and `]` characters, just add '\' before them like `\[` or `\]`

error: `[throttle]` cannot be resolved, ignoring it...
  --> /data/doc/code/tokio/tokio/src/util/stream.rs:13:48
   |
13 | /// Currently, there are only [`timeout`] and [`throttle`] functions, but
   |                                                ^^^^^^^^^^ cannot be resolved, ignoring
   |
   = help: to escape `[` and `]` characters, just add '\' before them like `\[` or `\]`

error: aborting due to 2 previous errors

error: Could not document `tokio`.

Caused by:
  process didn't exit successfully: `rustdoc --crate-name tokio /data/doc/code/tokio/tokio/src/lib.rs --color always -o /data/doc/code/thespis/thespis_impl/examples/single_thread/remote/target/doc --cfg 'feature="async-await-preview"' --cfg 'feature="bytes"' --cfg 'feature="codec"' --cfg 'feature="default"' --cfg 'feature="fs"' --cfg 'feature="io"' --cfg 'feature="mio"' --cfg 'feature="num_cpus"' --cfg 'feature="reactor"' --cfg 'feature="rt-full"' --cfg 'feature="sync"' --cfg 'feature="tcp"' --cfg 'feature="timer"' --cfg 'feature="tokio-async-await"' --cfg 'feature="tokio-codec"' --cfg 'feature="tokio-current-thread"' --cfg 'feature="tokio-executor"' --cfg 'feature="tokio-fs"' --cfg 'feature="tokio-io"' --cfg 'feature="tokio-reactor"' --cfg 'feature="tokio-sync"' --cfg 'feature="tokio-tcp"' --cfg 'feature="tokio-threadpool"' --cfg 'feature="tokio-timer"' --cfg 'feature="tokio-trace-core"' --cfg 'feature="tokio-udp"' --cfg 'feature="tokio-uds"' --cfg 'feature="udp"' --cfg 'feature="uds"' -L dependency=/data/doc/code/thespis/thespis_impl/examples/single_thread/remote/target/debug/deps --extern bytes=/data/doc/code/thespis/thespis_impl/examples/single_thread/remote/target/debug/deps/libbytes-20baa45fbd90af1a.rmeta --extern futures=/data/doc/code/thespis/thespis_impl/examples/single_thread/remote/target/debug/deps/libfutures-d7d88decd1b9f95b.rmeta --extern mio=/data/doc/code/thespis/thespis_impl/examples/single_thread/remote/target/debug/deps/libmio-a871dfd367e7dc96.rmeta --extern num_cpus=/data/doc/code/thespis/thespis_impl/examples/single_thread/remote/target/debug/deps/libnum_cpus-f3ff256cc843f32d.rmeta --extern tokio_async_await=/data/doc/code/thespis/thespis_impl/examples/single_thread/remote/target/debug/deps/libtokio_async_await-e3e8a0f8c821e5e8.rmeta --extern tokio_codec=/data/doc/code/thespis/thespis_impl/examples/single_thread/remote/target/debug/deps/libtokio_codec-1b4855f650f293cd.rmeta --extern tokio_current_thread=/data/doc/code/thespis/thespis_impl/examples/single_thread/remote/target/debug/deps/libtokio_current_thread-814b6491e95f409f.rmeta --extern tokio_executor=/data/doc/code/thespis/thespis_impl/examples/single_thread/remote/target/debug/deps/libtokio_executor-a0b701715912e0ed.rmeta --extern tokio_fs=/data/doc/code/thespis/thespis_impl/examples/single_thread/remote/target/debug/deps/libtokio_fs-2eeb9626da53135b.rmeta --extern tokio_io=/data/doc/code/thespis/thespis_impl/examples/single_thread/remote/target/debug/deps/libtokio_io-481680588c836c91.rmeta --extern tokio_reactor=/data/doc/code/thespis/thespis_impl/examples/single_thread/remote/target/debug/deps/libtokio_reactor-bb25cedc607922f1.rmeta --extern tokio_sync=/data/doc/code/thespis/thespis_impl/examples/single_thread/remote/target/debug/deps/libtokio_sync-b633bbfcc90c07e5.rmeta --extern tokio_tcp=/data/doc/code/thespis/thespis_impl/examples/single_thread/remote/target/debug/deps/libtokio_tcp-98a368eba6ccb1c0.rmeta --extern tokio_threadpool=/data/doc/code/thespis/thespis_impl/examples/single_thread/remote/target/debug/deps/libtokio_threadpool-2de3cad2cd252e63.rmeta --extern tokio_timer=/data/doc/code/thespis/thespis_impl/examples/single_thread/remote/target/debug/deps/libtokio_timer-d0620851afb193a3.rmeta --extern tokio_trace_core=/data/doc/code/thespis/thespis_impl/examples/single_thread/remote/target/debug/deps/libtokio_trace_core-b39affcf5303cc58.rmeta --extern tokio_udp=/data/doc/code/thespis/thespis_impl/examples/single_thread/remote/target/debug/deps/libtokio_udp-20fa1eeeed6b6d4f.rmeta --extern tokio_uds=/data/doc/code/thespis/thespis_impl/examples/single_thread/remote/target/debug/deps/libtokio_uds-2f0614c7fea4e5be.rmeta` (exit code: 1)
warning: build failed, waiting for other jobs to finish...
error: build failed
