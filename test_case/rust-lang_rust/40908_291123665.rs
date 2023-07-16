
	test
running `C:\projects\rust\build\ct\cargo\target\debug\cargo test -v`
thread 'test_release_ignore_panic' panicked at '
Expected: execs
    but: exited with exit code: 3221225477
--- stdout
--- stderr
   Compiling a v0.0.1 (file:///C:/projects/rust/build/ct/cargo/target/cit/t53/foo/a)
     Running `rustc --crate-name a a\src\lib.rs --crate-type lib --emit=dep-info,link -C debuginfo=2 -C metadata=0844ace4ff28f02a -C extra-filename=-0844ace4ff28f02a --out-dir C:\projects\rust\build\ct\cargo\target\cit\t53\foo\target\debug\deps -L dependency=C:\projects\rust\build\ct\cargo\target\cit\t53\foo\target\debug\deps`
   Compiling foo v0.0.1 (file:///C:/projects/rust/build/ct/cargo/target/cit/t53/foo)
     Running `rustc --crate-name foo src\lib.rs --emit=dep-info,link -C debuginfo=2 --test -C metadata=0a91976aa53e67d4 -C extra-filename=-0a91976aa53e67d4 --out-dir C:\projects\rust\build\ct\cargo\target\cit\t53\foo\target\debug\deps -L dependency=C:\projects\rust\build\ct\cargo\target\cit\t53\foo\target\debug\deps --extern a=C:\projects\rust\build\ct\cargo\target\cit\t53\foo\target\debug\deps\liba-0844ace4ff28f02a.rlib`
     Running `rustc --crate-name foo src\lib.rs --crate-type lib --emit=dep-info,link -C debuginfo=2 -C metadata=350a25faded805d4 -C extra-filename=-350a25faded805d4 --out-dir C:\projects\rust\build\ct\cargo\target\cit\t53\foo\target\debug\deps -L dependency=C:\projects\rust\build\ct\cargo\target\cit\t53\foo\target\debug\deps --extern a=C:\projects\rust\build\ct\cargo\target\cit\t53\foo\target\debug\deps\liba-0844ace4ff28f02a.rlib`
    Finished dev [unoptimized + debuginfo] target(s) in 0.64 secs
     Running `C:\projects\rust\build\ct\cargo\target\cit\t53\foo\target\debug\deps\foo-0a91976aa53e67d4.exe`
error: test failed
', C:\Users\appveyor\.cargo\registry\src\github.com-1ecc6299db9ec823\hamcrest-0.1.1\src\core.rs:31
note: Run with `RUST_BACKTRACE=1` for a backtrace.
failures:
    test_release_ignore_panic
test result: FAILED. 61 passed; 1 failed; 0 ignored; 0 measured
