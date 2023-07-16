
error[E0596]: cannot borrow `v` as mutable, as it is not declared as mutable
  --> src/main.rs:11:5
   |
11 |     f2(&mut v); // ERROR should be f2(v)
   |        ^^^^^^
   |        |
   |        cannot borrow as mutable
   |        try removing `&mut` here

error[E0596]: cannot borrow `v` as mutable, as it is not declared as mutable
  --> src/main.rs:16:5
   |
16 |     f3(&mut v); // ERROR should be f3(v)
   |        ^^^^^^
   |        |
   |        cannot borrow as mutable
   |        try removing `&mut` here
