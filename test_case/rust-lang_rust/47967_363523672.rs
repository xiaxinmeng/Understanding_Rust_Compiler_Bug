
warning: macro expansion includes fragment specifier
 --> confusing-macro.rs:2:22
  |
2 |     ($tt:tt) => { $tt:tt }
  |                      ^^^
help: if you meant to use the macro argument, do not include the fragment specifier:
  |
2 |     ($tt:tt) => { $tt }
  |                   ^^^
help: to suppress this warning, add a space after the type argument:
  |
2 |     ($tt:tt) => { $tt: tt }
  |                   ^^^^^^^
