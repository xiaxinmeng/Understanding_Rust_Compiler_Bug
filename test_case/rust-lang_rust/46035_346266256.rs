
---- changing_bin_features_caches_targets stdout ----
	running `C:\projects\rust\build\x86_64-pc-windows-msvc\stage2-tools\x86_64-pc-windows-msvc\release\cargo.exe run`
running `C:\projects\rust\build\x86_64-pc-windows-msvc\stage2-tools\x86_64-pc-windows-msvc\release\cargo.exe run --features foo`
thread 'changing_bin_features_caches_targets' panicked at '
Expected: execs
    but: exited with exit code: 101
--- stdout
--- stderr
   Compiling foo v0.0.1 (file:///C:/projects/rust/build/x86_64-pc-windows-msvc/stage2-tools/x86_64-pc-windows-msvc/cit/t3/foo)
error: failed to link or copy `C:\projects\rust\build\x86_64-pc-windows-msvc\stage2-tools\x86_64-pc-windows-msvc\cit\t3\foo\target\debug\deps\foo-f76c704c72dc1115.exe` to `C:\projects\rust\build\x86_64-pc-windows-msvc\stage2-tools\x86_64-pc-windows-msvc\cit\t3\foo\target\debug\foo.exe`
Caused by:
  Access is denied. (os error 5)
', C:\Users\appveyor\.cargo\registry\src\github.com-1ecc6299db9ec823\hamcrest-0.1.1\src\core.rs:31:12
note: Run with `RUST_BACKTRACE=1` for a backtrace.
---- changing_bin_paths_common_target_features_caches_targets stdout ----
	running `C:\projects\rust\build\x86_64-pc-windows-msvc\stage2-tools\x86_64-pc-windows-msvc\release\cargo.exe run`
running `C:\projects\rust\build\x86_64-pc-windows-msvc\stage2-tools\x86_64-pc-windows-msvc\release\cargo.exe clean -p a`
running `C:\projects\rust\build\x86_64-pc-windows-msvc\stage2-tools\x86_64-pc-windows-msvc\release\cargo.exe run`
thread 'changing_bin_paths_common_target_features_caches_targets' panicked at '
Expected: execs
    but: exited with exit code: 101
--- stdout
--- stderr
   Compiling a v0.0.1 (file:///C:/projects/rust/build/x86_64-pc-windows-msvc/stage2-tools/x86_64-pc-windows-msvc/cit/t2/foo/a)
error: failed to link or copy `C:\projects\rust\build\x86_64-pc-windows-msvc\stage2-tools\x86_64-pc-windows-msvc\cit\t2\foo\./target\debug\deps\a-a478fe7568e0eb90.exe` to `C:\projects\rust\build\x86_64-pc-windows-msvc\stage2-tools\x86_64-pc-windows-msvc\cit\t2\foo\./target\debug\a.exe`
Caused by:
  Access is denied. (os error 5)
', C:\Users\appveyor\.cargo\registry\src\github.com-1ecc6299db9ec823\hamcrest-0.1.1\src\core.rs:31:12
failures:
    changing_bin_features_caches_targets
    changing_bin_paths_common_target_features_caches_targets
