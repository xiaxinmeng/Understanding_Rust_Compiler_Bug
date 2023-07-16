
error[E0277]: the trait bound `S: Deref` is not satisfied
  --> src/main.rs:10:14
   |
10 |     Pin::new(S).x();
   |              ^ the trait `Deref` is not implemented for `S`
   |
   = note: required by `Pin::<P>::new`

error[E0599]: no method named `x` found for struct `Pin<S>` in the current scope
  --> src/main.rs:10:17
   |
10 |     Pin::new(S).x();
   |                 ^ method not found in `Pin<S>`
