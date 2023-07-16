
error[E0277]: `<() as Bar<&'a ()>>::Assoc` cannot be sent between threads safely
  --> src/lib.rs:16:5
   |
16 |     oops::<()>();
   |     ^^^^^^^^^^ `<() as Bar<&'a ()>>::Assoc` cannot be sent between threads safely
   |
   = help: the trait `for<'a> std::marker::Send` is not implemented for `<() as Bar<&'a ()>>::Assoc`
note: required by `oops`
  --> src/lib.rs:8:1
   |
8  | / fn oops<C>()
9  | | where
10 | |     for<'a> C: Bar<&'a ()>,
11 | |     for<'a> <C as Bar<&'a ()>>::Assoc: Send,
12 | | {
13 | | }
   | |_^
