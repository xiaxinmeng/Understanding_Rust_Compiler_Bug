
fn test<T>() {}
    
test::<(String)>();

warning: unnecessary parentheses around type
  --> src/main.rs:72:12
   |
72 |     test::<(String)>();
   |            ^      ^
   |
   = note: `#[warn(unused_parens)]` on by default
help: remove these parentheses
   |
72 -     test::<(String)>();
72 +     test::<String>();
