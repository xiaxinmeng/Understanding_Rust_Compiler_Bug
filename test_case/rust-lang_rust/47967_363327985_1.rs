rust
warning: macro expansion includes fragment specifier
 --> confusing-macro.rs:2:22
  |
2 |     ($tt:tt) => { $tt:tt }
  |                      ^^^
  |
  = help: did you mean `$tt`?
  = help: to suppress this warning, add a space: `$tt: tt`
