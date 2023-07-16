
error: expected one of `!`, `.`, `;`, `?`, `{`, `}`, or an operator, found `::`
  --> expr.rs:18:49
   |
18 |     match {macro_rules! call { ($t:path) => { $t::foo() } } call!(Foo);} {
   |                                                 ^^          ----------- in this macro invocation
   |                                                 |
   |                                                 expected one of 7 possible tokens here

