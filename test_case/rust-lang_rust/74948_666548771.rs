console
error[E0599]: no method named `as_deref_err` found for reference `&std::result::Result<u8, &i32>` in the current scope
   --> library/core/tests/result.rs:257:24
    |
257 |     assert_eq!(ref_err.as_deref_err(), expected_result);
    |                        ^^^^^^^^^^^^ method not found in `&std::result::Result<u8, &i32>`
