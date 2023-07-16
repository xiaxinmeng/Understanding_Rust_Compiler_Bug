
error[E0597]: borrowed value does not live long enough
  --> ALDS1_6_B.rs:14:5
   |
14 | /     writelnf!(
15 | |         "{}[{:d}]{}",
16 | |         join(f!(""), list::iter(slice!(a, [, pivot])).map(|e| f!("{e:d} "))),
17 | |         a[pivot],
18 | |         join(f!(""), list::iter(slice!(a, [pivot+1, ])).map(|e| f!(" {e:d}"))));
   | |                                                                                ^
   | |                                                                                |
   | |________________________________________________________________________________temporary value dropped here while still borrowed
   |                                                                                  temporary value does not live long enough
   |
   = note: values in a scope are dropped in the opposite order they are created
   = note: consider using a `let` binding to increase its lifetime
   = note: this error originates in a macro outside of the current crate (in Nightly builds, run with -Z external-macro-backtrace for more info)

error[E0597]: borrowed value does not live long enough
  --> ALDS1_6_B.rs:14:5
   |
14 | /     writelnf!(
15 | |         "{}[{:d}]{}",
16 | |         join(f!(""), list::iter(slice!(a, [, pivot])).map(|e| f!("{e:d} "))),
17 | |         a[pivot],
18 | |         join(f!(""), list::iter(slice!(a, [pivot+1, ])).map(|e| f!(" {e:d}"))));
   | |                                                                                ^
   | |                                                                                |
   | |________________________________________________________________________________temporary value dropped here while still borrowed
   |                                                                                  temporary value does not live long enough
   |
   = note: values in a scope are dropped in the opposite order they are created
   = note: consider using a `let` binding to increase its lifetime
   = note: this error originates in a macro outside of the current crate (in Nightly builds, run with -Z external-macro-backtrace for more info)

error[E0499]: cannot borrow `*porus_sink` as mutable more than once at a time
  --> ALDS1_6_B.rs:14:5
   |
14 | /     writelnf!(
15 | |         "{}[{:d}]{}",
16 | |         join(f!(""), list::iter(slice!(a, [, pivot])).map(|e| f!("{e:d} "))),
17 | |         a[pivot],
18 | |         join(f!(""), list::iter(slice!(a, [pivot+1, ])).map(|e| f!(" {e:d}"))));
   | |                                                                                ^
   | |                                                                                |
   | |________________________________________________________________________________mutable borrow ends here
   |                                                                                  mutable borrow starts here in previous iteration of loop

error: aborting due to 3 previous errors

Some errors occurred: E0499, E0597.
For more information about an error, try `rustc --explain E0499`.
