
warning: unnecessary lifetime parameter `'a`
  --> file.rs:11:14
   |
11 | fn _foo3<'c, 'a: 'static, 'b: 'a, 'd>(_x: &'a str) -> &'a str {
   |              ^^^^^^^^^^^
help: you can use the `'static` lifetime directly, in place of `'a`
   |
11 | fn _foo3<'c, 'b: 'a, 'd>(_x: &'static str) -> &'static str {
   |             --                ^^^^^^^
