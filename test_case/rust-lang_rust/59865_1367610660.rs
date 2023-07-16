
error: lifetime may not live long enough
  --> src/lib.rs:12:9
   |
8  | impl<'a> Iterator for AnIterator<'a> {
   |      -- lifetime `'a` defined here
...
11 |     fn next<'b>(&'b mut self) -> Option<Self::Item> {
   |             -- lifetime `'b` defined here
12 |         Some(self.first.as_str())
   |         ^^^^^^^^^^^^^^^^^^^^^^^^^ associated function was supposed to return data with lifetime `'a` but it is returning data with lifetime `'b`
   |
   = help: consider adding the following bound: `'b: 'a`
