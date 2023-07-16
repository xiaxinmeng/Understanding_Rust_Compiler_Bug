
error: chained comparison operators require parentheses
  --> file.rs:16:25
   |
16 |     println!("{:?}", Vec<i32>::new());
   |                         ^^^^^^^ incorrect type argument syntax
   |
hint: use `::<...>` instead of `<...>` if you meant to specify type arguments
16 |     println!("{:?}", Vec::<i32>::new());
   |                         ^^^^^^^^^ 
