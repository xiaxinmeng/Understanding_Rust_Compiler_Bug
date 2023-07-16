
error[E0271]: type mismatch resolving `for<'a> <for<'a> fn(&'a ()) -> <() as LifetimeToType<'a>>::Out {id} as std::ops::FnOnce<(&'a (),)>>::Output == <() as LifetimeToType<'a>>::Out`
  --> src/main.rs:16:5
   |
13 | fn assert_fn<F: for<'a> FnOnce(&'a ()) -> <() as LifetimeToType<'a>>::Out>(_func: F) { }
   |    ---------                              ------------------------------- required by this bound in `assert_fn`
...
16 |     assert_fn(id);
   |     ^^^^^^^^^ expected associated type, found `&()`
   |
   = note: expected associated type `<() as LifetimeToType<'_>>::Out`
                    found reference `&()`
   = note: consider constraining the associated type `<() as LifetimeToType<'_>>::Out` to `&()` or calling a method that returns `<() as LifetimeToType<'_>>::Out`
   = note: for more information, visit https://doc.rust-lang.org/book/ch19-03-advanced-traits.html
