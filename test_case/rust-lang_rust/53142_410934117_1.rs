
error[E0597]: borrowed value does not live long enough
  --> ITP1_6_A.rs:11:5
   |
11 |     writelnf!("{}", join(f!(" "), list::iter(slice!(a, [,,-1])).map(|e| f!("{e:d}"))));
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^-
   |     |                                                                                 |
   |     |                                                                                 temporary value dropped here while still borrowed
   |     temporary value does not live long enough
   |
   = note: values in a scope are dropped in the opposite order they are created
   = note: consider using a `let` binding to increase its lifetime
   = note: this error originates in a macro outside of the current crate (in Nightly builds, run with -Z external-macro-backtrace for more info)
