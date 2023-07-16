
failures:

---- cyclic_dev_dep_doc_test stdout ----
	running `C:\bot\slave\auto-win-msvc-64-cargotest\build\obj\build\ct\cargo\target\debug\cargo test`
thread 'cyclic_dev_dep_doc_test' panicked at '
Expected: execs
    but: differences:
  6 - |running 1 test|
    + |running 0 tests|

  7 - |test _0 ... ok|
    + ||

  8 - ||
    + |test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured|

  9 - |test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured|
    + ||

 10 - ||
    +


other output:
`   Compiling foo v0.0.1 (file:///C:/bot/slave/auto-win-msvc-64-cargotest/build/obj/build/ct/cargo/target/cit/t14/foo)
   Compiling bar v0.0.1 (file:///C:/bot/slave/auto-win-msvc-64-cargotest/build/obj/build/ct/cargo/target/cit/t14/foo/bar)
    Finished debug [unoptimized + debuginfo] target(s) in 1.3 secs
     Running target\debug\deps\foo-6688213a3cfc5207.exe
   Doc-tests foo
`', C:/msys64/slave/auto-win-msvc-64-cargotest/cargo-home\registry\src\github.com-1ecc6299db9ec823\hamcrest-0.1.1\src\core.rs:31
note: Run with `RUST_BACKTRACE=1` for a backtrace.


failures:
    cyclic_dev_dep_doc_test

test result: FAILED. 54 passed; 1 failed; 0 ignored; 0 measured



command did not execute successfully: "C:\\bot\\slave\\auto-win-msvc-64-cargotest\\build\\obj\\build\\x86_64-pc-windows-msvc\\stage2-tools\\x86_64-pc-windows-msvc\\release\\cargotest.exe" "C:\\bot\\slave\\auto-win-msvc-64-cargotest\\build\\obj\\build\\x86_64-pc-windows-msvc\\stage0/bin\\cargo.exe" "C:\\bot\\slave\\auto-win-msvc-64-cargotest\\build\\obj\\build\\ct"
expected success, got: exit code: 101
