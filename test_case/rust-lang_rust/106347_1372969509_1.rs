text
error[E0061]: this function takes 1 argument but 3 arguments were supplied
 --> <anon>:7:5
  |
7 |     foo(1, 2, "hello");
  |     ^^^ -  - argument of type `{integer}` unexpected
  |         |
  |         argument of type `{integer}` unexpected
  |
note: function defined here
 --> <anon>:2:4
  |
2 | fn foo(x: &str) {
  |    ^^^ -------
help: remove the extra arguments
  |
7 -     foo(1, 2, "hello");
7 +     foo(, "hello");
  |
