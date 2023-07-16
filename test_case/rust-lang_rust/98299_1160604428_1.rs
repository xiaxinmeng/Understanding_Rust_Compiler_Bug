
error[E0282]: type annotations needed
 --> <source>:4:31
  |
4 |     SmallCString::try_from(p).map(|cstr| cstr);
  |                               ^^^ cannot infer type for enum `Result<SmallCString<{_: usize}>, ()>`
