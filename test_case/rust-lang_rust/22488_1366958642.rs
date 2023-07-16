
error[E0423]: cannot initialize a tuple struct which contains private fields
 --> src/main.rs:7:19
  |
7 |         wtf: Some(Box(U {
  |                   ^^^
  |
note: constructor is not visible here due to private fields
 --> /rustc/92c1937a90e5b6f20fa6e87016d6869da363972e/library/alloc/src/boxed.rs:202:14
  |
  = note: private field
  |
  = note: private field
help: consider importing one of these items instead
  |
1 | use syn::Expr::Box;
  |
1 | use syn::Pat::Box;
  |
1 | use syn::token::Box;
  |
