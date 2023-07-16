
[01:33:06] ---- test_release_ignore_panic stdout ----
[01:33:06] 	test
[01:33:06] running `C:\projects\rust\build\ct\cargo\target\debug\cargo test -v`
[01:33:06] bench
[01:33:06] running `C:\projects\rust\build\ct\cargo\target\debug\cargo bench -v`
[01:33:06] thread 'test_release_ignore_panic' panicked at '
[01:33:06] Expected: execs
[01:33:06]     but: exited with exit code: 3221225477
[01:33:06] --- stdout
[01:33:06] 
[01:33:06] --- stderr
[01:33:06]    Compiling a v0.0.1 (file:///C:/projects/rust/build/ct/cargo/target/cit/t53/foo/a)
[01:33:06]      Running `rustc --crate-name a a\src\lib.rs --crate-type lib --emit=dep-info,link -C opt-level=3 -C metadata=850c3cbe05004df2 -C extra-filename=-850c3cbe05004df2 --out-dir C:\projects\rust\build\ct\cargo\target\cit\t53\foo\target\release\deps -L dependency=C:\projects\rust\build\ct\cargo\target\cit\t53\foo\target\release\deps`
[01:33:06]    Compiling foo v0.0.1 (file:///C:/projects/rust/build/ct/cargo/target/cit/t53/foo)
[01:33:06]      Running `rustc --crate-name foo src\lib.rs --emit=dep-info,link -C opt-level=3 --test -C metadata=4f826724f99492b5 -C extra-filename=-4f826724f99492b5 --out-dir C:\projects\rust\build\ct\cargo\target\cit\t53\foo\target\release\deps -L dependency=C:\projects\rust\build\ct\cargo\target\cit\t53\foo\target\release\deps --extern a=C:\projects\rust\build\ct\cargo\target\cit\t53\foo\target\release\deps\liba-850c3cbe05004df2.rlib`
[01:33:06]     Finished release [optimized] target(s) in 0.61 secs
[01:33:06]      Running `C:\projects\rust\build\ct\cargo\target\cit\t53\foo\target\release\deps\foo-4f826724f99492b5.exe --bench`
[01:33:06] error: bench failed
[01:33:06] ', C:\Users\appveyor\.cargo\registry\src\github.com-1ecc6299db9ec823\hamcrest-0.1.1\src\core.rs:31
[01:33:06] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[01:33:06] 
[01:33:06] 
[01:33:06] failures:
[01:33:06]     test_release_ignore_panic
[01:33:06] 
[01:33:06] test result: FAILED. 61 passed; 1 failed; 0 ignored; 0 measured
[01:33:06] 
[01:33:06] error: test failed
[01:33:06] thread 'main' panicked at 'tests failed for https://github.com/rust-lang/cargo', src\tools\cargotest\main.rs:98
[01:33:06] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[01:33:06] 
[01:33:06] 
[01:33:06] command did not execute successfully: "C:\\projects\\rust\\build\\x86_64-pc-windows-msvc\\stage0-tools\\x86_64-pc-windows-msvc\\release\\cargotest.exe" "C:\\projects\\rust\\build\\x86_64-pc-windows-msvc\\stage0/bin\\cargo.exe" "C:\\projects\\rust\\build\\ct"
[01:33:06] expected success, got: exit code: 101
