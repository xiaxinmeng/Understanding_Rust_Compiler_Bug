text
error[E0507]: cannot move out of `*it` which is behind a shared reference
  --> src/main.rs:11:12
   |
11 |     *it == *it;
   |            ^^^ move occurs because `*it` has type `Box<dyn T>`, which does not implement the `Copy` trait

error: aborting due to previous error
