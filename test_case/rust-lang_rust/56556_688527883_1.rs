
error[E0277]: `<() as Bar<'a>>::Assoc` cannot be sent between threads safely
  --> src/main.rs:17:5
   |
9  | fn oops<C>()
   |    ---- required by a bound in this
...
12 |     for<'a> <C as Bar<'a>>::Assoc: Send,
   |                                    ---- required by this bound in `oops`
...
17 |     oops::<()>();
   |     ^^^^^^^^^^ `<() as Bar<'a>>::Assoc` cannot be sent between threads safely
   |
   = help: the trait `for<'a> std::marker::Send` is not implemented for `<() as Bar<'a>>::Assoc`
