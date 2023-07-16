
error[E0697]: cannot use `static` keyword on closures
  --> $DIR/E0697.rs:2:5
   |
LL |     static || {};
   |     ^^^^^^^^^ help: to make the closure capture its environment by
   |               value and have a `'static` lifetime, use the `move`
   |               keyword: `move || {}`
   = note: for more on capturing environment with closures, read https://doc.rust-lang.org/book/ch13-01-closures.html#capturing-the-environment-with-closures
