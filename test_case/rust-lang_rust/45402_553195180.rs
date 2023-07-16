rust
8  |     fn get_mut(&mut self) -> &mut Something {
   |                - let's call the lifetime of this reference `'1`
9  |         for _ in self.list.iter() {
10 |             let a = &mut self.thing;
   |                     ^^^^^^^^^^^^^^^ mutable borrow starts here in previous iteration of loop
11 |             if true {
12 |                 return a;
   |                        - returning this value requires that `self.thing` is borrowed for `'1`
