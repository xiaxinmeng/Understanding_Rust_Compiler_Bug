plain
    |
note: the function `utf8_char_width` is defined here
   --> library/core/src/str/validations.rs:373:1
    |
373 | const fn utf8_char_width(byte: u8) -> usize {

error[E0603]: function `utf8_char_width` is private
   --> library/core/src/str/mod.rs:74:40
    |
    |
74  | pub use validations::{next_code_point, utf8_char_width};
    |
note: the function `utf8_char_width` is defined here
   --> library/core/src/str/validations.rs:373:1
    |
    |
373 | const fn utf8_char_width(byte: u8) -> usize {

For more information about this error, try `rustc --explain E0603`.
error: could not compile `core` (lib) due to 2 previous errors
Build completed unsuccessfully in 0:00:14
