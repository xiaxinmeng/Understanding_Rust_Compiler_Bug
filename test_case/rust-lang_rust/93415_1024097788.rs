
warning: positional argument in format string refers to named argument
 --> <source>:3:18
  |
3 |     println!("{a}{}", a=what);
  |                  ^^
help: use its name to avoid confusion
  |
3 |     println!("{a}{a}", a=what);
  |                   +
