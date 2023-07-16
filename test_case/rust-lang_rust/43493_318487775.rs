
$ cargo script -u trace_macros -e 'trace_macros!(true); macro_rules! m { (x $($t:tt)*) => { m!() } } m!(foo);'
   Compiling expr v0.1.0 (file:///Users/alex/.cargo/.cargo/script-cache/expr-634e13f197b6cae5)
error: no rules expected the token `foo`
  --> .cargo/.cargo/script-cache/expr-634e13f197b6cae5/expr.rs:11:81
   |
11 |     match {trace_macros!(true); macro_rules! m { (x $($t:tt)*) => { m!() } } m!(foo);} {
   |                                                                                 ^^^
   = note: expanding `m! { foo }`
