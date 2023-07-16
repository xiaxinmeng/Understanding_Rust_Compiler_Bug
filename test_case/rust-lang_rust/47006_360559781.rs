bash
./x.py build --incremental -j 5
Updating submodules
downloading https://static.rust-lang.org/dist/2018-01-02/rust-std-beta-x86_64-unknown-linux-gnu.tar.gz
######################################################################## 100.0%
extracting /home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/build/cache/2018-01-02/rust-std-beta-x86_64-unknown-linux-gnu.tar.gz
downloading https://static.rust-lang.org/dist/2018-01-02/rustc-beta-x86_64-unknown-linux-gnu.tar.gz
######################################################################## 100.0%
extracting /home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/build/cache/2018-01-02/rustc-beta-x86_64-unknown-linux-gnu.tar.gz
   Compiling num-traits v0.1.41
   Compiling lazy_static v0.2.11
   Compiling getopts v0.2.15
   Compiling itoa v0.3.4
   Compiling unicode-xid v0.0.4
   Compiling dtoa v0.4.2
   Compiling quote v0.3.15
   Compiling cc v1.0.4
   Compiling serde v1.0.27
   Compiling libc v0.2.36
   Compiling cfg-if v0.1.2
   Compiling synom v0.11.3
   Compiling syn v0.11.11
   Compiling filetime v0.1.15
   Compiling num_cpus v1.8.0
   Compiling cmake v0.1.29
   Compiling build_helper v0.1.0 (file:///home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/src/build_helper)
   Compiling serde_derive_internals v0.19.0
   Compiling serde_derive v1.0.27
   Compiling toml v0.4.5
   Compiling serde_json v1.0.9
   Compiling bootstrap v0.0.0 (file:///home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/src/bootstrap)
    Finished dev [unoptimized] target(s) in 75.69 secs
Building stage0 std artifacts (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
   Compiling unwind v0.0.0 (file:///home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/src/libunwind)
   Compiling core v0.0.0 (file:///home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/src/libcore)
   Compiling cfg-if v0.1.2
   Compiling libc v0.2.36
   Compiling cc v1.0.4
   Compiling filetime v0.1.15
   Compiling build_helper v0.1.0 (file:///home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/src/build_helper)
   Compiling std v0.0.0 (file:///home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/src/libstd)
   Compiling alloc_jemalloc v0.0.0 (file:///home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/src/liballoc_jemalloc)
   Compiling compiler_builtins v0.0.0 (file:///home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/src/rustc/compiler_builtins_shim)
   Compiling cmake v0.1.29
   Compiling rustc_tsan v0.0.0 (file:///home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/src/librustc_tsan)
   Compiling rustc_lsan v0.0.0 (file:///home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/src/librustc_lsan)
   Compiling rustc_asan v0.0.0 (file:///home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/src/librustc_asan)
   Compiling rustc_msan v0.0.0 (file:///home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/src/librustc_msan)
   Compiling libc v0.0.0 (file:///home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/src/rustc/libc_shim)
   Compiling std_unicode v0.0.0 (file:///home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/src/libstd_unicode)
   Compiling panic_abort v0.0.0 (file:///home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/src/libpanic_abort)
   Compiling alloc v0.0.0 (file:///home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/src/liballoc)
   Compiling panic_unwind v0.0.0 (file:///home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/src/libpanic_unwind)
   Compiling alloc_system v0.0.0 (file:///home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/src/liballoc_system)
error: the struct `#[repr(align(u16))]` attribute is experimental (see issue #33626)
  --> libstd/sync/mpsc/cache_aligned.rs:14:1
   |
14 | #[repr(align(64))]
   | ^^^^^^^^^^^^^^^^^^
   |
   = help: add #![feature(repr_align)] to the crate attributes to enable

error: aborting due to previous error

error: Could not compile `std`.

Caused by:
  process didn't exit successfully: `/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/build/bootstrap/debug/rustc --crate-name std libstd/lib.rs --error-format json --crate-type dylib --crate-type rlib --emit=dep-info,link -C prefer-dynamic -C opt-level=2 --cfg feature="alloc_jemalloc" --cfg feature="backtrace" --cfg feature="jemalloc" --cfg feature="panic-unwind" --cfg feature="panic_unwind" -C metadata=2d4d16d24b0a1c7c -C extra-filename=-2d4d16d24b0a1c7c --out-dir /home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps --target x86_64-unknown-linux-gnu -L dependency=/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps -L dependency=/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage0-std/release/deps --extern std_unicode=/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps/libstd_unicode-0d7cd53a0f8ca1cb.rlib --extern libc=/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps/liblibc-fb0532659c2770d1.rlib --extern rustc_lsan=/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps/librustc_lsan-49988d7fcfe221d9.rlib --extern panic_unwind=/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps/libpanic_unwind-ceb6d2604463fa9a.rlib --extern alloc=/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps/liballoc-2fa1908321f62402.rlib --extern core=/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps/libcore-81500df0e121824a.rlib --extern rustc_msan=/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps/librustc_msan-e792ac012c81d5de.rlib --extern compiler_builtins=/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps/libcompiler_builtins-31f820ea1f446abe.rlib --extern alloc_system=/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps/liballoc_system-045f19d2833d64ad.rlib --extern rustc_asan=/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps/librustc_asan-ae57686ea748ec53.rlib --extern panic_abort=/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps/libpanic_abort-df8ccc0bb4380645.rlib --extern unwind=/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps/libunwind-52a24300225aab4a.rlib --extern alloc_jemalloc=/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps/liballoc_jemalloc-c8af342068974a5a.rlib --extern rustc_tsan=/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps/librustc_tsan-0794e094bc2cb159.rlib -L native=/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/native/libbacktrace/.libs -l static=backtrace -l dl -l rt -l pthread -L native=/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/native/jemalloc/lib -L native=/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/build/compiler_builtins-5d7fc88585475c99/out` (exit code: 101)
thread 'main' panicked at 'command did not execute successfully: "/home/xftroxgpx/.cargo/bin/cargo" "build" "--target" "x86_64-unknown-linux-gnu" "--release" "--features" "panic-unwind jemalloc backtrace" "--manifest-path" "/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/src/libstd/Cargo.toml" "--message-format" "json"
expected success, got: exit code: 101', bootstrap/compile.rs:881:9
note: Run with `RUST_BACKTRACE=1` for a backtrace.
failed to run: /home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/build/bootstrap/debug/bootstrap build --incremental -j 5
Build completed unsuccessfully in 0:03:10

real	3m11.358s
user	5m35.554s
sys	0m18.022s

