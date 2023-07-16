
---- Rust_Compiler_Error_Index_55 stdout ----
    error: only foreign functions are allowed to be variadic
 --> <anon>:8:18
  |
8 | fn foo(x: u8, ...) {}
  |                  ^

error: aborting due to previous error

thread 'Rust_Compiler_Error_Index_55' panicked at 'Box<Any>', src/librustc_errors/lib.rs:461
thread 'Rust_Compiler_Error_Index_55' panicked at 'Some expected error codes were not found: ["E0045"]', src/librustdoc/test.rs:293

