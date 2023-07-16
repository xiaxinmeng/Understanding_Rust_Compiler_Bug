rs
 --> src/main.rs:4:5
  |
1 |   struct Foo {
  |          --- while parsing this struct
...
4 | /     fn inc(&mut self) {
5 | |         self.x += 1;
6 | |     }
  | |_____^
  |
  = help: unlike in C++, Java, and C#, functions are declared in `impl` blocks
  = help: see https://doc.rust-lang.org/book/ch05-03-method-syntax.html for more information
