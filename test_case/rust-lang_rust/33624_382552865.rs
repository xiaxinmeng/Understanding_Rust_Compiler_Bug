
error[E0195]: lifetime parameters or bounds on method `a` do not match the trait declaration
 --> src/main.rs:8:5
  |
2 |     fn a<'b>(self) -> Box<A<'b>>;
  |     ----------------------------- lifetimes in impl do not match this method in trait
...
8 |     fn a<'b>(self) -> Box<A<'a>> {}
  |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^ lifetimes do not match method in trait
