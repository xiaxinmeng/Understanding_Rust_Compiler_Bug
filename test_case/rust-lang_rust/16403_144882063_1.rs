
<anon>:3:9: 3:28 warning: function is never used: `foo`, #[warn(dead_code)] on by default
<anon>:3         extern fn foo() { }
                 ^~~~~~~~~~~~~~~~~~~
<anon>:7:9: 7:39 warning: function is never used: `foo`, #[warn(dead_code)] on by default
<anon>:7         extern fn foo(_bar: usize) { }
                 ^~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
error: symbol `_ZN4main3foo10__rust_abiE` already defined
playpen: application terminated with error code 101
