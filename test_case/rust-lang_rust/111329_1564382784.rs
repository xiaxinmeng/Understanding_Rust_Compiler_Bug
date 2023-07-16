plain
error: test failed, to rerun pass `--lib`

failures:

---- abis::test_version_check stdout ----
thread 'abis::test_version_check' panicked at 'called `Result::unwrap()` on an `Err` value: Custom { kind: InvalidData, error: "unsupported metadata version 8" }', crates/proc-macro-srv/src/abis/mod.rs:144:55

---- tests::list_test_macros stdout ----
---- tests::list_test_macros stdout ----
thread 'tests::list_test_macros' panicked at 'called `Result::unwrap()` on an `Err` value: "Cannot create expander for /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/proc-macro-test-b10b33ad22ec78c7/out/target/debug/libproc_macro_test_impl.so: unsupported metadata version 8"', crates/proc-macro-srv/src/tests/utils.rs:39:44
---- tests::test_derive_empty stdout ----
---- tests::test_derive_empty stdout ----
thread 'tests::test_derive_empty' panicked at 'called `Result::unwrap()` on an `Err` value: Io(Custom { kind: InvalidData, error: "unsupported metadata version 8" })', crates/proc-macro-srv/src/tests/utils.rs:28:48
---- tests::test_attr_macro stdout ----
---- tests::test_attr_macro stdout ----
thread 'tests::test_attr_macro' panicked at 'called `Result::unwrap()` on an `Err` value: Io(Custom { kind: InvalidData, error: "unsupported metadata version 8" })', crates/proc-macro-srv/src/tests/utils.rs:28:48
---- tests::test_derive_error stdout ----
---- tests::test_derive_error stdout ----
thread 'tests::test_derive_error' panicked at 'called `Result::unwrap()` on an `Err` value: Io(Custom { kind: InvalidData, error: "unsupported metadata version 8" })', crates/proc-macro-srv/src/tests/utils.rs:28:48
---- tests::test_fn_like_macro_clone_ident_subtree stdout ----
---- tests::test_fn_like_macro_clone_ident_subtree stdout ----
thread 'tests::test_fn_like_macro_clone_ident_subtree' panicked at 'called `Result::unwrap()` on an `Err` value: Io(Custom { kind: InvalidData, error: "unsupported metadata version 8" })', crates/proc-macro-srv/src/tests/utils.rs:28:48
---- tests::test_fn_like_macro_noop stdout ----
---- tests::test_fn_like_macro_noop stdout ----
thread 'tests::test_fn_like_macro_noop' panicked at 'called `Result::unwrap()` on an `Err` value: Io(Custom { kind: InvalidData, error: "unsupported metadata version 8" })', crates/proc-macro-srv/src/tests/utils.rs:28:48

---- tests::test_fn_like_macro_clone_literals stdout ----
---- tests::test_fn_like_macro_clone_literals stdout ----
thread 'tests::test_fn_like_macro_clone_literals' panicked at 'called `Result::unwrap()` on an `Err` value: Io(Custom { kind: InvalidData, error: "unsupported metadata version 8" })', crates/proc-macro-srv/src/tests/utils.rs:28:48
---- tests::test_fn_like_macro_clone_raw_ident stdout ----
---- tests::test_fn_like_macro_clone_raw_ident stdout ----
thread 'tests::test_fn_like_macro_clone_raw_ident' panicked at 'called `Result::unwrap()` on an `Err` value: Io(Custom { kind: InvalidData, error: "unsupported metadata version 8" })', crates/proc-macro-srv/src/tests/utils.rs:28:48
---- tests::test_fn_like_mk_idents stdout ----
---- tests::test_fn_like_mk_idents stdout ----
thread 'tests::test_fn_like_mk_idents' panicked at 'called `Result::unwrap()` on an `Err` value: Io(Custom { kind: InvalidData, error: "unsupported metadata version 8" })', crates/proc-macro-srv/src/tests/utils.rs:28:48
---- tests::test_fn_like_mk_literals stdout ----
---- tests::test_fn_like_mk_literals stdout ----
thread 'tests::test_fn_like_mk_literals' panicked at 'called `Result::unwrap()` on an `Err` value: Io(Custom { kind: InvalidData, error: "unsupported metadata version 8" })', crates/proc-macro-srv/src/tests/utils.rs:28:48

failures:
    abis::test_version_check
    tests::list_test_macros
