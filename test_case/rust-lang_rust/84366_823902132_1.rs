rust
error[E0495]: cannot infer an appropriate lifetime due to conflicting requirements
  --> <source>:24:36
   |
24 |     static_transfers_to_associated(&f::<'a>, s)
   |                                    ^^^^^^^^
   |
note: first, the lifetime cannot outlive the lifetime 'a as defined on the function body at 20:1...
  --> <source>:20:1
   |
20 | / fn make_static_displayable<'a>(s: &'a str) -> Box<fmt::Display> {
21 | |     //let f = || -> &'a str { "" };
22 | |     fn f<'a>() -> &'a str { "" }
23 | |     // problem is: the type of `f::<'a>` is 'static
24 | |     static_transfers_to_associated(&f::<'a>, s)
25 | | }
   | |_^
note: ...so that reference does not outlive borrowed content
  --> <source>:24:46
   |
24 |     static_transfers_to_associated(&f::<'a>, s)
   |                                              ^
   = note: but, the lifetime must be valid for the static lifetime...
note: ...so that the type `fn() -> &str {make_static_displayable::f::<'a>}` will meet its required lifetime bounds
  --> <source>:24:5
   |
24 |     static_transfers_to_associated(&f::<'a>, s)
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

error: aborting due to previous error(s)

Compiler returned: 101
