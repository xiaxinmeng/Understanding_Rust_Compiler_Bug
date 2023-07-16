text
error: no rules expected the token `"bug"`
  --> src\lib.rs:16:28
   |
4  |         macro_rules! lookup {
   |         ------------------- when calling this macro
...
16 |     println!("{}", lookup!("bug"));
   |                            ^^^^^ no rules expected this token in macro call

