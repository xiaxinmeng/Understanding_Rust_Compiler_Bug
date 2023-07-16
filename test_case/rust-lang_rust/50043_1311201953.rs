
error: reached the recursion limit while instantiating `print::<PhantomData<PhantomData<...>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>`
  --> src/main.rs:14:32
   |
14 |         Expression::Node(e) => print::<Wrapper<S>>(*e),
   |                                ^^^^^^^^^^^^^^^^^^^^^^^
   |
note: `print` defined here
  --> src/main.rs:11:1
   |
11 | fn print<S: Serializer>(e: Expression) {
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
   = note: the full type name has been written to '/playground/target/debug/deps/playground-07b5fd22867752b9.long-type.txt'
