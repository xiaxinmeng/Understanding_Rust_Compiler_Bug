bash
$ ./x.py build -j 5
Updating submodules
    Finished dev [unoptimized] target(s) in 0.0 secs
Building stage0 std artifacts (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
   Compiling std v0.0.0 (file:///home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/src/libstd)
error[E0658]: the struct `#[repr(align(u16))]` attribute is experimental (see issue #33626)
  --> libstd/sync/mpsc/cache_aligned.rs:14:1
   |
14 | #[repr(align(64))]
   | ^^^^^^^^^^^^^^^^^^
   |
   = help: add #![feature(repr_align)] to the crate attributes to enable

error: aborting due to previous error

error: Could not compile `std`.

Caused by:
  process didn't exit successfully: `/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/build/bootstrap/debug/rustc --crate-name std libstd/lib.rs --error-format json --crate-type dylib --crate-type rlib --emit=dep-info,link -C prefer-dynamic -C opt-level=2 --cfg feature="alloc_jemalloc" --cfg feature="backtrace" --cfg feature="jemalloc" --cfg feature="panic-unwind" --cfg feature="panic_unwind" -C metadata=f7765955cd67cf2b -C extra-filename=-f7765955cd67cf2b --out-dir /home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps --target x86_64-unknown-linux-gnu -L dependency=/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps -L dependency=/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage0-std/release/deps --extern compiler_builtins=/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps/libcompiler_builtins-f1b239799398f922.rlib --extern libc=/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps/liblibc-e8c93db2d9f7b1a1.rlib --extern alloc_system=/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps/liballoc_system-c02aaefcdf3ed014.rlib --extern alloc_jemalloc=/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps/liballoc_jemalloc-2fbac0da8986f50b.rlib --extern unwind=/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps/libunwind-73b86850bc2ec62b.rlib --extern core=/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps/libcore-1fa9134e95dd93e2.rlib --extern rustc_msan=/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps/librustc_msan-62258ea7aa2b71fe.rlib --extern panic_abort=/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps/libpanic_abort-a0a3317b1f888bcd.rlib --extern std_unicode=/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps/libstd_unicode-d98c7e52ed02392c.rlib --extern panic_unwind=/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps/libpanic_unwind-00b9a6cd0d4fc568.rlib --extern alloc=/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps/liballoc-4d746674fccddbc7.rlib --extern rustc_asan=/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps/librustc_asan-7dee8df74f43bae5.rlib --extern rustc_lsan=/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps/librustc_lsan-431a8de9030d5943.rlib --extern rustc_tsan=/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps/librustc_tsan-dec54e9bf41c7f34.rlib -L native=/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/native/libbacktrace/.libs -l static=backtrace -l dl -l rt -l pthread -L native=/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/native/jemalloc/lib -L native=/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/build/compiler_builtins-f322bdafb8005573/out` (exit code: 101)
thread 'main' panicked at 'command did not execute successfully: "/home/xftroxgpx/.cargo/bin/cargo" "build" "--target" "x86_64-unknown-linux-gnu" "--release" "--features" "panic-unwind jemalloc backtrace" "--manifest-path" "/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/src/libstd/Cargo.toml" "--message-format" "json"
expected success, got: exit code: 101', bootstrap/compile.rs:881:9
note: Run with `RUST_BACKTRACE=1` for a backtrace.
failed to run: /home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/build/bootstrap/debug/bootstrap build -j 5
Build completed unsuccessfully in 0:00:09

real	0m10.357s
user	0m8.365s
sys	0m2.077s

