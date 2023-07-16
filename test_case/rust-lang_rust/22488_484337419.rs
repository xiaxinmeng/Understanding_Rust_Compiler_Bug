
error[E0423]: expected function, found struct `Box`
 --> src/main.rs:5:15
  |
5 |  U{ wtf: Some(Box(U{ wtf: None })) };
  |               ^^^ constructor is not visible here due to private fields
help: possible better candidates are found in other modules, you can import them into scope
  |
1 | use syn::Expr::Box;
  |
1 | use syn::Pat::Box;
  |
1 | use syn::token::Box;
  |

error[E0392]: parameter `T` is never used
 --> src/main.rs:1:11
  |
1 | struct U <T> {
  |           ^ unused type parameter
  |
  = help: consider removing `T` or using a marker such as `std::marker::PhantomData`

error: aborting due to 2 previous errors
