
$ RUSTC_LOG=trace rustc +stage1 src/main.rs
 INFO 2019-07-11T13:33:39Z: jobserver::imp: created a jobserver: Client { read: File { fd: 3, path: "pipe:[2538076]", read: true, write: false }, write: File { fd: 4, path: "pipe:[2538076]", read: false, write: true } }
 INFO 2019-07-11T13:33:39Z: rustc_interface::util: codegen backend candidate: /home/lampam/asd/clone/rust/build/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/codegen-backends
 INFO 2019-07-11T13:33:39Z: rustc_interface::util: probing /home/lampam/asd/clone/rust/build/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/codegen-backends for a codegen backend
error: this file contains an un-closed delimiter
 --> src/main.rs:1:11
  |
1 | fn main((ؼ
  |        -- ^
  |        ||
  |        |un-closed delimiter
  |        un-closed delimiter

thread 'rustc' panicked at 'byte index 10 is not a char boundary; it is inside 'ؼ' (bytes 9..11) of `fn main((ؼ`', src/libcore/str/mod.rs:2039:5
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
error: aborting due to previous error
