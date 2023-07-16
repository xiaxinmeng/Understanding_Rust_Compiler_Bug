
$ rustc +devel file.rs -Zborrowck=mir
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

error: aborting due to previous error
