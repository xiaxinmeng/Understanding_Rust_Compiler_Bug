\n\nThis error indicates that the compiler was unable to sensibly evaluate a\nconstant expression that had to be evaluated. Attempting to divide by 0\nor causing an integer overflow are two ways to induce this error.\n\nEnsure that the expressions given can be evaluated as the desired integer type.\n\nSee the [Custom Discriminants][custom-discriminants] section of the Reference\nfor more information about setting custom integer types on fieldless enums\nusing the [`repr` attribute][repr-attribute].\n\n[custom-discriminants]: https://doc.rust-lang.org/reference/items/enumerations.html#custom-discriminant-values-for-field-less-enumerations\n[repr-attribute]: https://doc.rust-lang.org/reference/type-layout.html#reprc-enums\n"},"level":"error","spans":[{"file_name":"tests/ui/indexing_slicing_index.rs","byte_start":430,"byte_end":441,"line_start":10,"line_end":10,"column_start":24,"column_end":35,"is_primary":true,"text":[{"text":"const REF_ERR: &i32 = &ARR[idx4()]; // Ok, let rustc handle const contexts.","highlight_start":24,"highlight_end":35}],"label":"index out of bounds: the length is 2 but the index is 4","suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error[E0080]: evaluation of constant value failed\n  --> tests/ui/indexing_slicing_index.rs:10:24\n   |\nLL | const REF_ERR: &i32 = &ARR[idx4()]; // Ok, let rustc handle const contexts.\n   |                        ^^^^^^^^^^^ index out of bounds: the length is 2 but the index is 4\n\n"}
{"message":"aborting due to 9 previous errors","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to 9 previous errors\n\n"}
{"message":"For more information about this error, try `rustc --explain E0080`.","code":null,"level":"failure-note","spans":[],"children":[],"rendered":"For more information about this error, try `rustc --explain E0080`.\n"}

------------------------------------------

thread 'compile_test' panicked at 'Some tests failed', /home/r/.cargo/registry/src/github.com-1ecc6299db9ec823/compiletest_rs-0.8.0/src/lib.rs:111:22
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace


failures:
    compile_test

test result: FAILED. 2 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out; finished in 29.45s

error: test failed, to rerun pass '--test compile-test'
   Compiling shell-escape v0.1.5
   Compiling indoc v1.0.6
   Compiling opener v0.5.0
   Compiling clippy_dev v0.0.1 (/home/r/src/rust/rustc.3/src/tools/clippy/clippy_dev)
    Finished release [optimized + debuginfo] target(s) in 7.91s
     Running `/home/r/src/rust/rustc.3/build/x86_64-unknown-linux-gnu/stage1-tools/x86_64-unknown-linux-gnu/release/clippy_dev bless`
updating tests/ui/indexing_slicing_index.stderr
Build completed successfully in 0:00:39
