
Updating submodules
    Finished dev [unoptimized] target(s) in 0.0 secs
Building stage0 std artifacts (x86_64-pc-windows-msvc -> x86_64-pc-windows-msvc)
    Finished release [optimized] target(s) in 0.0 secs
Copying stage0 std from stage0 (x86_64-pc-windows-msvc -> x86_64-pc-windows-msvc / x86_64-pc-windows-msvc)
Building stage0 test artifacts (x86_64-pc-windows-msvc -> x86_64-pc-windows-msvc)
    Finished release [optimized] target(s) in 0.0 secs
Copying stage0 test from stage0 (x86_64-pc-windows-msvc -> x86_64-pc-windows-msvc / x86_64-pc-windows-msvc)
Building stage0 compiler artifacts (x86_64-pc-windows-msvc -> x86_64-pc-windows-msvc)
   Compiling rustc v0.0.0 (file:///C:/rust/src/librustc)
   Compiling rustc_incremental v0.0.0 (file:///C:/rust/src/librustc_incremental)
   Compiling rustc_typeck v0.0.0 (file:///C:/rust/src/librustc_typeck)
   Compiling rustc_metadata v0.0.0 (file:///C:/rust/src/librustc_metadata)
   Compiling rustc_allocator v0.0.0 (file:///C:/rust/src/librustc_allocator)
   Compiling rustc_trans_utils v0.0.0 (file:///C:/rust/src/librustc_trans_utils)
   Compiling rustc_resolve v0.0.0 (file:///C:/rust/src/librustc_resolve)
   Compiling rustc_const_eval v0.0.0 (file:///C:/rust/src/librustc_const_eval)
   Compiling rustc_plugin v0.0.0 (file:///C:/rust/src/librustc_plugin)
   Compiling rustc_passes v0.0.0 (file:///C:/rust/src/librustc_passes)
   Compiling rustc_lint v0.0.0 (file:///C:/rust/src/librustc_lint)
   Compiling rustc_mir v0.0.0 (file:///C:/rust/src/librustc_mir)
   Compiling rustc_privacy v0.0.0 (file:///C:/rust/src/librustc_privacy)
   Compiling rustc_save_analysis v0.0.0 (file:///C:/rust/src/librustc_save_analysis)
   Compiling rustc_trans v0.0.0 (file:///C:/rust/src/librustc_trans)
   Compiling rustc_borrowck v0.0.0 (file:///C:/rust/src/librustc_borrowck)
   Compiling rustc_driver v0.0.0 (file:///C:/rust/src/librustc_driver)
   Compiling rustc-main v0.0.0 (file:///C:/rust/src/rustc)
    Finished release [optimized] target(s) in 207.72 secs
Copying stage0 rustc from stage0 (x86_64-pc-windows-msvc -> x86_64-pc-windows-msvc / x86_64-pc-windows-msvc)
Assembling stage1 compiler (x86_64-pc-windows-msvc)
Building stage1 std artifacts (x86_64-pc-windows-msvc -> x86_64-pc-windows-msvc)
   Compiling cc v1.0.4
   Compiling core v0.0.0 (file:///C:/rust/src/libcore)
   Compiling unwind v0.0.0 (file:///C:/rust/src/libunwind)
   Compiling cfg-if v0.1.2
   Compiling filetime v0.1.15
   Compiling build_helper v0.1.0 (file:///C:/rust/src/build_helper)
   Compiling std v0.0.0 (file:///C:/rust/src/libstd)
   Compiling compiler_builtins v0.0.0 (file:///C:/rust/src/rustc/compiler_builtins_shim)
error[E0119]: conflicting implementations of trait `iter_private::TrustedRandomAccess` for type `iter::Cloned<_>`:
   --> libcore\iter\mod.rs:593:1
    |
581 | / default unsafe impl<'a, I, T: 'a> TrustedRandomAccess for Cloned<I>
582 | |     where I: TrustedRandomAccess<Item=&'a T>, T: Clone
583 | | {
584 | |     unsafe fn get_unchecked(&mut self, i: usize) -> Self::Item {
...   |
589 | |     fn may_have_side_effect() -> bool { true }
590 | | }
    | |_- first implementation here
...
593 | / unsafe impl<'a, I, T: 'a> TrustedRandomAccess for Cloned<I>
594 | |     where I: TrustedRandomAccess<Item=&'a T>, T: Copy
595 | | {
596 | |     unsafe fn get_unchecked(&mut self, i: usize) -> Self::Item {
...   |
601 | |     fn may_have_side_effect() -> bool { false }
602 | | }
    | |_^ conflicting implementation for `iter::Cloned<_>`

error: aborting due to previous error

error: Could not compile `core`.

Caused by:
  process didn't exit successfully: `C:\rust\build\bootstrap/debug/rustc --crate-name core libcore\lib.rs --error-format json --crate-type lib --emit=dep-info,link -C opt-level=2 -C metadata=10ce4b4b7843f587 -C extra-filename=-10ce4b4b7843f587 --out-dir C:\rust\build\x86_64-pc-windows-msvc\stage1-std\x86_64-pc-windows-msvc\release\deps --target x86_64-pc-windows-msvc -L dependency=C:\rust\build\x86_64-pc-windows-msvc\stage1-std\x86_64-pc-windows-msvc\release\deps -L dependency=C:\rust\build\x86_64-pc-windows-msvc\stage1-std\release\deps` (exit code: 101)
thread 'main' panicked at 'command did not execute successfully: "C:\\rust\\build\\x86_64-pc-windows-msvc\\stage0/bin\\cargo.exe" "build" "--target" "x86_64-pc-windows-msvc" "-j" "4" "--release" "--features" "panic-unwind backtrace" "--manifest-path" "C:\\rust\\src/libstd/Cargo.toml" "--message-format" "json"
expected success, got: exit code: 101', bootstrap\compile.rs:881:9
note: Run with `RUST_BACKTRACE=1` for a backtrace.
failed to run: C:\rust\build\bootstrap\debug\bootstrap build src/libstd --stage 1 --incremental
Build completed unsuccessfully in 0:05:00
