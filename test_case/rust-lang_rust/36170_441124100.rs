
error[E0499]: cannot borrow `self.map` as mutable more than once at a time
  --> src/main.rs:16:9
   |
10 |     fn get_or_allocate(&mut self, k: &str) -> &mut usize {
   |                        - let's call the lifetime of this reference `'1`
11 |         {
12 |             if let Some(v) = self.map.get_mut(k) {
   |                              -------- first mutable borrow occurs here
13 |                 return v;
   |                        - returning this value requires that `self.map` is borrowed for `'1`
...
16 |         self.map.entry(String::from(k)).or_insert(0)
   |         ^^^^^^^^ second mutable borrow occurs here
