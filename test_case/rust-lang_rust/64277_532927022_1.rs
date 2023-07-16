
error[E0423]: expected value, found struct `A`
  --> src/main.rs:62:25
   |
62 |     if let _ = (A {}) + A {} {} // `contains_exterior_struct_lit` says this should require parens!
   |                         ^
help: a local variable with a similar name exists
   |
62 |     if let _ = (A {}) + x {} {} // `contains_exterior_struct_lit` says this should require parens!
   |                         ^
help: surround the struct literal with parenthesis
   |
62 |     if let _ = (A {}) + (A {}) {} // `contains_exterior_struct_lit` says this should require parens!
   |                         ^^^^^^
help: possible better candidates are found in other modules, you can import them into scope
   |
5  | use futures::future::Either::A;
   |
5  | use tokio::prelude::future::Either::A;
