
error[E0596]: cannot borrow immutable borrowed content `*x` as mutable
 --> check-access-lvalue.rs:3:19
  |
3 |     let rw = &mut *x;
  |                   ^^ cannot borrow as mutable

error: aborting due to previous error
