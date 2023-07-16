
$ rustc +devel file.rs -Zborrowck=compare
error[E0597]: `self.first` does not live long enough (Ast)
  --> file.rs:12:14
   |
12 |         Some(self.first.as_str())
   |              ^^^^^^^^^^ borrowed value does not live long enough
   |
   = note: borrowed value must be valid for the static lifetime...
note: ...but borrowed value is only valid for the lifetime 'b as defined on the method body at 11:13
  --> file.rs:11:13
   |
11 |     fn next<'b>(&'b mut self) -> Option<Self::Item> {
   |             ^^

error: lifetime may not live long enough
  --> file.rs:12:9
   |
8  | impl<'a> Iterator for AnIterator<'a> {
   |      -- lifetime `'a` defined here
...
11 |     fn next<'b>(&'b mut self) -> Option<Self::Item> {
   |             -- lifetime `'b` defined here
12 |         Some(self.first.as_str())
   |         ^^^^^^^^^^^^^^^^^^^^^^^^^ returning this value requires that `'b` must outlive `'a`

error: aborting due to 2 previous errors

For more information about this error, try `rustc --explain E0597`.
