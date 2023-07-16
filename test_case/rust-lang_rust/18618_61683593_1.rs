
<anon>:1:1: 1:12 warning: function is never used: `foo`, #[warn(dead_code)] on by default
<anon>:1 fn foo() {}
         ^~~~~~~~~~~
<anon>:3:1: 5:2 warning: function is never used: `bar`, #[warn(dead_code)] on by default
<anon>:3 fn bar() {
<anon>:4     foo()
<anon>:5 }
