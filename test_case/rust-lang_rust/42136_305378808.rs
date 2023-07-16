
[01:25:51] failures:
[01:25:51]
[01:25:51] ---- cargo_compile_simple_git_dep stdout ----
[01:25:51]  running `C:\projects\rust\build\x86_64-pc-windows-msvc\stage2-tools\x86_64-pc-windows-msvc\release\cargo.exe build`
[01:25:51] running `C:\projects\rust\build\x86_64-pc-windows-msvc\stage2-tools\x86_64-pc-windows-msvc\cit\t3\foo\target\debug\foo.exe`
[01:25:51] thread 'cargo_compile_simple_git_dep' panicked at '
[01:25:51] Expected: execs
[01:25:51]     but: differences:
[01:25:51]   0 - |hello world|
[01:25:51]     +
[01:25:51]
[01:25:51]
[01:25:51] other output:
[01:25:51] ``', C:\Users\appveyor\.cargo\registry\src\github.com-1ecc6299db9ec823\hamcrest-0.1.1\src\core.rs:31
[01:25:51] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[01:25:51]
[01:25:51]
[01:25:51] failures:
[01:25:51]     cargo_compile_simple_git_dep
[01:25:51]
[01:25:51] test result: FAILED. 32 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out
