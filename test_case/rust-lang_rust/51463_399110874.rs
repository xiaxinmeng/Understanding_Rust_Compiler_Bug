
---- freshness::changing_bin_features_caches_targets stdout ----
running `C:\projects\rust\build\x86_64-pc-windows-msvc\stage2-tools\x86_64-pc-windows-msvc\release\cargo.exe build`
running `C:\projects\rust\build\x86_64-pc-windows-msvc\stage2-tools\x86_64-pc-windows-msvc\cit\t543\foo\target\debug\off1.exe`
running `C:\projects\rust\build\x86_64-pc-windows-msvc\stage2-tools\x86_64-pc-windows-msvc\release\cargo.exe build --features foo`
running `C:\projects\rust\build\x86_64-pc-windows-msvc\stage2-tools\x86_64-pc-windows-msvc\cit\t543\foo\target\debug\on1.exe`
running `C:\projects\rust\build\x86_64-pc-windows-msvc\stage2-tools\x86_64-pc-windows-msvc\release\cargo.exe build`
thread 'freshness::changing_bin_features_caches_targets' panicked at '
Expected: execs
    but: exited with exit code: 101
--- stdout
--- stderr
error: failed to remove file `C:\projects\rust\build\x86_64-pc-windows-msvc\stage2-tools\x86_64-pc-windows-msvc\cit\t543\foo\target\debug\foo.exe`
Caused by:
  Access is denied. (os error 5)
', tools\cargo\tests\testsuite\hamcrest.rs:13:9
note: Run with `RUST_BACKTRACE=1` for a backtrace.
failures:
    freshness::changing_bin_features_caches_targets
test result: FAILED. 1306 passed; 1 failed; 4 ignored; 0 measured; 0 filtered out
