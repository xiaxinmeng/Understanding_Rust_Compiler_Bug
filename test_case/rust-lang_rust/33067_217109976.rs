
---- test_cargo_freshness::rebuild_tests_if_lib_changes stdout ----
    thread 'test_cargo_freshness::rebuild_tests_if_lib_changes' panicked at 'called `Result::unwrap()` on an `Err` value: Error { repr: Os { code: 32, message: "The process cannot access the file because it is being used by another process." } }', C:\bot\slave\auto-win-msvc-64-cargotest\build\src\libcore\result.rs:785
note: Run with `RUST_BACKTRACE=1` for a backtrace.


failures:
    test_cargo_freshness::rebuild_tests_if_lib_changes
