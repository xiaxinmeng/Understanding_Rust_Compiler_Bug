
error[E0597]: `a` does not live long enough
 --> src/main.rs:3:14
  |
3 |     let b = &a[0];
  |              ^ borrowed value does not live long enough
4 | 
5 |     panic!(b);
  |     ---------- argument requires that `a` is borrowed for `'static`
6 | }
  | - `a` dropped here while still borrowed
  |
  = note: this error originates in a macro outside of the current crate (in Nightly builds, run with -Z external-macro-backtrace for more info)
