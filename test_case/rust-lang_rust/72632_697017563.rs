plain
[RUSTC-TIMING] rustc_resolve test:false 127.764
[RUSTC-TIMING] rustc_plugin_impl test:false 13.873
[RUSTC-TIMING] rustc_privacy test:false 49.555
   Compiling rustc_interface v0.0.0 (D:\a\rust\rust\compiler\rustc_interface)
Assertion failed: ((((rwlock_t *)*rwl)->valid == LIFE_RWLOCK) && (((rwlock_t *)*rwl)->busy > 0)), file ../../src/mingw-w64/mingw-w64-libraries/winpthreads/src/rwlock.c, line 40
error: could not compile `rustc_interface`.

To learn more, run the command again with --verbose.



command did not execute successfully: "D:\\a\\rust\\rust\\build\\i686-pc-windows-gnu\\stage0\\bin\\cargo.exe" "test" "--target" "i686-pc-windows-gnu" "-Zbinary-dep-depinfo" "-j" "8" "--release" "--locked" "--color" "always" "--features" " llvm max_level_info" "--manifest-path" "D:\\a\\rust\\rust\\compiler/rustc/Cargo.toml" "-p" "rustc_interface" "--"


failed to run: D:\a\rust\rust\build\bootstrap\debug\bootstrap test --stage 2 --exclude src/test/ui --exclude src/test/compile-fail
Build completed unsuccessfully in 0:45:08
Build completed unsuccessfully in 0:45:08
make: *** [Makefile:82: ci-mingw-subset-1] Error 1
  local time: Sat Sep 19 23:05:35 CUT 2020
  network time: Sat, 19 Sep 2020 23:05:35 GMT
== end clock drift check ==
== end clock drift check ==
##[error]Process completed with exit code 2.
Terminate orphan process: pid (3068) (node)
Terminate orphan process: pid (7104) (python)
Terminate orphan process: pid (3936) (sccache)
