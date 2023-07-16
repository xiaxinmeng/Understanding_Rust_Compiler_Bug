
error[E0277]: the trait bound `String: From<&&str>` is not satisfied
  --> src/main.rs:21:17
   |
1  | fn foo(x: u32, a: impl Into<String>, y: u32, b: impl Into<String>) {
   |                                                      ------------ required by this bound in `foo`
...
21 |         foo(42, a, 43, b);
   |                 ^ the trait `From<&&str>` is not implemented for `String`
   |
   = help: the following implementations were found:
             <String as From<&String>>
             <String as From<&mut str>>
             <String as From<&str>>
             <String as From<Box<str>>>
           and 2 others
   = note: required because of the requirements on the impl of `Into<String>` for `&&str`
