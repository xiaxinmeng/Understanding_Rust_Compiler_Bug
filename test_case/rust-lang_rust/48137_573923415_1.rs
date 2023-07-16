text
error: no rules expected the token `,`
  --> src/lib.rs:10:10
   |
1  | macro_rules! foo {
   | ---------------- when calling this macro
...
10 | foo!(Name, fn foo() { println!("Hello world"); });
   |          ^ no rules expected this token in macro call
