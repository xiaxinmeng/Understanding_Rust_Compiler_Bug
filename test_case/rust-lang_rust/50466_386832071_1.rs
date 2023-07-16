text
        Finished dev [unoptimized] target(s) in 0.0 secs
    Building stage0 std artifacts (x86_64-apple-darwin -> x86_64-apple-darwin)
    Compiling core v0.0.0 (file:///$DIR/rust/src/libcore)

    ...

    Testing core stage0 (x86_64-apple-darwin -> x86_64-apple-darwin)
    Compiling libc v0.2.40
    Compiling rand v0.4.2
    Compiling core v0.0.0 (file:///$DIR/rust/src/libcore)
        Finished release [optimized] target(s) in 59.69 secs
        Running build/x86_64-apple-darwin/stage0-std/x86_64-apple-darwin/release/deps/core-82cbac52c153f0e4

    running 786 tests
    test any::any_downcast_mut ... ok
    test any::any_downcast_ref ... ok
    test any::any_fixed_vec ... ok
    test any::any_owning ... ok
    test any::any_referenced ... ok
    test any::any_unsized ... ok
    test array::array_try_from ... ok
    test array::fixed_size_array ... ok
    test ascii::inference_works ... ok
    test ascii::test_is_ascii ... ok
    test ascii::test_is_ascii_alphabetic ... ok
    test ascii::test_is_ascii_alphanumeric ... ok
    test ascii::test_is_ascii_control ... ok
    test ascii::test_is_ascii_digit ... ok
    test ascii::test_eq_ignore_ascii_case ... ok
    test ascii::test_is_ascii_graphic ... ok
    test ascii::test_is_ascii_hexdigit ... ok
    test ascii::test_is_ascii_lowercase ... ok
    test ascii::test_is_ascii_punctuation ... ok
    test ascii::test_is_ascii_uppercase ... ok
    test ascii::test_is_ascii_whitespace ... ok
    test ascii::test_make_ascii_lower_case ... ok
    test ascii::test_make_ascii_upper_case ... FAILED
    test atomic::bool_ ... ok
    test atomic::bool_and ... ok
    test atomic::bool_nand ... ok

    ...

    failures:

    ---- ascii::test_make_ascii_upper_case stdout ----
        thread 'ascii::test_make_ascii_upper_case' panicked at 'assertion failed: `(left == right)`
    left: `'*'`,
    right: `'A'`', libcore/../libcore/tests/ascii.rs:93:5
    note: Run with `RUST_BACKTRACE=1` for a backtrace.


    failures:
        ascii::test_make_ascii_upper_case

    test result: FAILED. 783 passed; 1 failed; 2 ignored; 0 measured; 0 filtered out

    error: test failed, to rerun pass '--test coretests'
    