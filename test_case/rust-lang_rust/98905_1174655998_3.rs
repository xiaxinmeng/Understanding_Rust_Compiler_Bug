
error[E0117]: only traits defined in the current crate can be implemented for types defined outside of the crate
  --> src/main.rs:13:1
   |
13 | impl<'a> AsRef<Path> for Cow<'a, CurDir> {
   | ^^^^^^^^^-----------^^^^^---------------
   | |        |               |
   | |        |               `Cow` is not defined in the current crate
   | |        `Path` is not defined in the current crate
   | impl doesn't use only types from inside the current crate
   |
   = note: define and implement a trait or new type instead

For more information about this error, try `rustc --explain E0117`.
