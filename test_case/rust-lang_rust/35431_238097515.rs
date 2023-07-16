

---- Rust_Compiler_Error_Index_329 stdout ----
    <anon>:3:1: 8:2 warning: enum is never used: `Foo`, #[warn(dead_code)] on by default 
<anon>:3 enum Foo {
         ^
<anon>:4:9: 4:27 error: literal out of range for isize 
<anon>:4     X = 0x7fffffffffffffff,
                 ^~~~~~~~~~~~~~~~~~
<anon>:2:12: 2:32 note: lint level defined here 
<anon>:2     #[deny(overflowing_literals)]
                    ^~~~~~~~~~~~~~~~~~~~
thread 'Rust_Compiler_Error_Index_329' panicked at 'Some expected error codes were not found: ["E0370"]', C:\bot\slave\auto-win-gnu-32-opt-rustbuild\build\src\librustdoc\test.rs:297


failures:
    Rust_Compiler_Error_Index_329
