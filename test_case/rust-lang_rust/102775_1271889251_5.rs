
error[E0282]: type annotations needed
  --> src/main.rs:10:22
   |
10 |     let u: u32 = a[m.into()];
   |                      ^^^^
   |
help: try using a fully qualified path to specify the expected types
   |
10 |     let u: u32 = a[<M as Into<T>>::into(m)];
   |                    +++++++++++++++++++++ ~
