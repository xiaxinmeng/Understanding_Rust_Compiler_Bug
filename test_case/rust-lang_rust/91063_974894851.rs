
error[E0277]: cannot add `char` to `{integer}`
 --> src/main.rs:2:15
  |
2 |     let x = 5 + '0';
  |               ^ no implementation for `{integer} + char`
  |
  = help: the trait `Add<char>` is not implemented for `{integer}`
  
error[E0369]: cannot add `u32` to `char`
 --> src/main.rs:2:17
  |
2 |     let x = '5' + 5u32 + '0';
  |             --- ^ ---- u32
  |             |
  |             char
