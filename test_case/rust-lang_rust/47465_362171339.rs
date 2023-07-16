
Compiling style_traits v0.0.1 (file:///C:/projects/rust/build/ct/servo/components/style_traits)
thread 'rustc' panicked at 'byte index 13 is out of bounds of `MallocSizeOf`', libcore\str\mod.rs:2221:9
note: Run with `RUST_BACKTRACE=1` for a backtrace.
error: internal compiler error: unexpected panic
note: the compiler unexpectedly panicked. this is a bug.
note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
note: rustc 1.25.0-dev running on x86_64-pc-windows-msvc
error: Could not compile `style_traits`.
warning: build failed, waiting for other jobs to finish...
error: build failed
thread 'main' panicked at 'tests failed for https://github.com/servo/servo', tools\cargotest\main.rs:100:9
note: Run with `RUST_BACKTRACE=1` for a backtrace.
command did not execute successfully: "C:\\projects\\rust\\build\\x86_64-pc-windows-msvc\\stage0-tools-bin\\cargotest.exe" "C:\\projects\\rust\\build\\x86_64-pc-windows-msvc\\stage0/bin\\cargo.exe" "C:\\projects\\rust\\build\\ct"
expected success, got: exit code: 101
failed to run: C:\projects\rust\build\bootstrap\debug\bootstrap test src/tools/cargo src/tools/cargotest src/test/pretty src/test/run-pass/pretty src/test/run-fail/pretty src/test/run-pass-valgrind/pretty src/test/run-pass-fulldeps/pretty src/test/run-fail-fulldeps/pretty
Build completed unsuccessfully in 0:33:42
make: *** [Makefile:54: check-aux] Error 1
Command exited with code 2
cat %CD%\sccache.log || exit 0
WARN:sccache::cache::s3: Got AWS error: Error(BadHTTPStatus(NotFound), State { next_error: None })
