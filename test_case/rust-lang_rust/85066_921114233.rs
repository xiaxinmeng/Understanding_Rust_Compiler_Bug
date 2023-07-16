`
#![feature(extended_key_value_attributes)]
#![doc = include_str!("../not_existing_file.md")]
struct Documented {}
//~^^ ERROR 2:10: 2:49: couldn't read /home/<username>/dev/rustc/rust/src/test/ui/attributes/../not_existing_file.md: No such file or directory (os error 2)

fn main() {}
