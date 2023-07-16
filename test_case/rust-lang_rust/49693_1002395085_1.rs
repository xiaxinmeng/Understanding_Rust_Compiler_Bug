
error[E0507]: cannot move out of `c.inner` which is behind a shared reference
  --> src/main.rs:12:28
   |
12 |     let _ = option.map(|c| c.inner);
   |                            ^^^^^^^ move occurs because `c.inner` has type `NotCopy`, which does not implement the `Copy` trait
