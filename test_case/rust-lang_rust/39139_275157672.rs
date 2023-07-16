
error: cannot borrow immutable borrowed content `*self.s` as mutable
  --> /checkout/src/test/ui/did_you_mean/issue-38147-3.rs:17:9
   |
12 |     s: &'a String
   |     ------------- use `&'a mut String` here to make mutable
...
16 |     fn f(&self) {
   |          ----- use `&mut self` here to make mutable
17 |         self.s.push('x');
   |         ^^^^^^ cannot borrow as mutable

error: aborting due to previous error
