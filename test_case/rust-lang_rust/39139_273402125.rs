
error: cannot borrow immutable borrowed content `*self.s` as mutable
  --> $DIR/issue-38147-3.rs:17:9
   |
12 |     s: &'a String
   |     ------------- this field should be `&mut`
...|
16 |     fn f(&self) {
   |          -----  use `&mut self` here to make mutable
17 |         self.s.push('x');
   |         ^^^^^^
