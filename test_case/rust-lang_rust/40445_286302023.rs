
error: cannot borrow immutable field `z.x` as mutable
  --> $DIR/issue-39544.rs:21:18
   |
20 |     let z = Z { x: X::Y };
   |         - consider changing this to `mut z`
21 |     let _ = &mut z.x;
   |                  ^^^ cannot mutably borrow immutable field
