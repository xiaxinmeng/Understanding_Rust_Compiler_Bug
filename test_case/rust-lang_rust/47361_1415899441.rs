
error[E0425]: cannot find function `vav_os` in module `env`
 --> src/main.rs:4:10
  |
4 |     env::vav_os("PATH");
  |          ^^^^^^ help: a function with a similar name exists: `var_os`
 --> /rustc/f3126500f25114ba4e0ac3e76694dd45a22de56d/library/std/src/env.rs:268:1
  |
  = note: similarly named function `var_os` defined here
