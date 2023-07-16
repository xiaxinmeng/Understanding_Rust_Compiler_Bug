
error[E0596]: cannot borrow immutable item `self` as mutable
  --> src/main.rs:19:9
   |
19 |         (&mut self).bar(); //~ ERROR cannot borrow
   |         ^^^^^^^^^^^ cannot borrow as mutable

