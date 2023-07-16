
error: cannot borrow immutable borrowed content `*self.s` as mutable
  --> $DIR/issue-38147-3.rs:17:9
17 |         self.s.push('x');
   |         ^^^^^^ cannot borrow as mutable
