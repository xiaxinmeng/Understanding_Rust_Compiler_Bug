
error: lifetime may not live long enough
  --> src/main.rs:42:9
   |
37 | impl<'a, V:'a> Iterator for MutableIter<'a,V> {
   |      -- lifetime `'a` defined here
38 |     type Item = &'a mut V;
39 |     fn next(&mut self) -> Option<&'a mut V> {
   |             - let's call the lifetime of this reference `'1`
...
42 |         Some(&mut self.cont.item)
   |         ^^^^^^^^^^^^^^^^^^^^^^^^^ associated function was supposed to return data with lifetime `'a` but it is returning data with lifetime `'1`
