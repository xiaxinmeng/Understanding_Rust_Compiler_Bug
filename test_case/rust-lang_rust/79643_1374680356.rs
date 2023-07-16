
error: `impl` item signature doesn't match `trait` item signature
 --> src/lib.rs:6:5
  |
6 |     fn from(foo: &Foo) -> &u32 {
  |     ^^^^^^^^^^^^^^^^^^^^^^^^^^ found `fn(&'1 Foo) -> &'1 u32`
  |
  = note: expected `fn(&'1 Foo) -> &'2 u32`
             found `fn(&'1 Foo) -> &'1 u32`
  = help: the lifetime requirements from the `impl` do not correspond to the requirements in the `trait`
  = help: verify the lifetime relationships in the `trait` and `impl` between the `self` argument, the other inputs and its output
