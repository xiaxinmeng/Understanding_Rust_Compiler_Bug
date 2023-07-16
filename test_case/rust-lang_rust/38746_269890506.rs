
INFO:rustc_metadata::creader: resolving crate `extern crate rustc_trans as rustc_trans`
INFO:rustc_metadata::creader: falling back to a load
INFO:rustc_metadata::locator: lib candidate: /home/mark/Edit/rust-build/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_trans-04b9ea62d71b4818.so
INFO:rustc_metadata::locator: lib candidate: /home/mark/Edit/rust-build/build/x86_64-unknown-linux-gnu/stage0-sysroot/lib/rustlib/x86_64-unknown-linux-gnu/lib/librustc_trans-04b9ea62d71b4818.so
INFO:rustc_metadata::locator: dylib reading metadata from: /home/mark/Edit/rust-build/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_trans-04b9ea62d71b4818.so
INFO:rustc_metadata::locator: reading "librustc_trans-04b9ea62d71b4818.so" => Duration { secs: 0, nanos: 4787878 }
INFO:rustc_metadata::locator: Rejecting via hash: expected 42ede6cbc4bc35c2 got 463ac8f69f29c717
INFO:rustc_metadata::locator: metadata mismatch
INFO:rustc_metadata::locator: dylib reading metadata from: /home/mark/Edit/rust-build/build/x86_64-unknown-linux-gnu/stage0-sysroot/lib/rustlib/x86_64-unknown-linux-gnu/lib/librustc_trans-04b9ea62d71b4818.so
INFO:rustc_metadata::locator: reading "librustc_trans-04b9ea62d71b4818.so" => Duration { secs: 0, nanos: 4798029 }
INFO:rustc_metadata::creader: register crate `extern crate rustc_trans as rustc_trans`
error[E0523]: found two different crates with name `rustc_trans` that are not distinguished by differing `-C metadata`. This will result in symbol conflicts between the two.
  --> src/rustc/rustdoc.rs:13:1
   |
13 | extern crate rustdoc;
   | ^^^^^^^^^^^^^^^^^^^^^

error: Could not compile `rustc-main`.

Caused by:
  process didn't exit successfully: `/home/mark/Edit/rust-build/build/bootstrap/debug/rustc --crate-name rustdoc src/rustc/rustdoc.rs --crate-type bin -C opt-level=2 --cfg feature="jemalloc" --cfg feature="rustc_back" -C metadata=7e166da6bf0b63c0 -C extra-filename=-7e166da6bf0b63c0 --out-dir /home/mark/Edit/rust-build/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps --emit=dep-info,link --target x86_64-unknown-linux-gnu -L dependency=/home/mark/Edit/rust-build/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps -L dependency=/home/mark/Edit/rust-build/build/x86_64-unknown-linux-gnu/stage0-rustc/release/deps --extern rustc_driver=/home/mark/Edit/rust-build/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_driver-7f15bfb21db92eb9.so --extern rustdoc=/home/mark/Edit/rust-build/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustdoc-6a966222823ef7f9.so --extern rustc_back=/home/mark/Edit/rust-build/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_back-26a0a47ac4c092b8.so -L native=/home/mark/Edit/rust-build/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/build/flate-4885c71e4cad27aa/out -L native=/home/mark/Edit/rust-build/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/build/rustc_llvm-4fef95dc29ee60a4/out -L native=/home/mark/Edit/rust-build/build/x86_64-unknown-linux-gnu/llvm/lib -L native=/home/mark/Edit/rust-build/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/build/rustdoc-8c070a2dffcbc259/out` (exit code: 101)
