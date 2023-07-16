text
error[E0423]: `MyStruct` is the name of a struct or struct variant, but this expression uses it like a function name
 --> <anon>:4:5
  |
4 |     MyStruct.foo;
  |     ^^^^^^^^ struct called like a function
  |
  = help: did you mean to write: `MyStruct { /* fields */ }`?
