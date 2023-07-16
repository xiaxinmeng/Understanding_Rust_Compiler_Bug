
error[[E0277]](https://doc.rust-lang.org/nightly/error-index.html#E0277): the trait bound `S: Deref` is not satisfied
  --> src/main.rs:10:14
   |
10 |     Pin::new(S).x();
   |     -------- ^ the trait `Deref` is not implemented for `S`
   |     |
   |     required by a bound introduced by this call
   |
note: required by a bound in `Pin::<P>::new`
  --> /rustc/659e169d37990b9c730a59a96081f2ef7afbe8f1/library/core/src/pin.rs:501:5
help: consider borrowing here
   |
10 |     Pin::new(&S).x();
   |              +
10 |     Pin::new(&mut S).x();
   |              ++++

error[[E0599]](https://doc.rust-lang.org/nightly/error-index.html#E0599): no method named `x` found for struct `Pin` in the current scope
  --> src/main.rs:10:17
   |
10 |     Pin::new(S).x();
   |                 ^ method not found in `Pin<S>`
