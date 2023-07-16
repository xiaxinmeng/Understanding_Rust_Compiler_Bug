
error: internal compiler error: /home/r/src/rust/rustc/src/librustc_middle/macros.rs:13:35: erroneous constant not captured by required_consts
  --> /home/r/src/rust/rustc/src/test/ui/consts/promoted_div_by_zero.rs:8:13
   |
LL |     let x = &(1 / (1 - 1));
   |             ^^^^^^^^^^^^^^
