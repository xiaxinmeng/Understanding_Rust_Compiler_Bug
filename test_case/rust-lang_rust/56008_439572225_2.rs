
error[E0119]: conflicting implementations of trait `std::marker::Copy` for type `SpookyThing<_>`:
  --> src/main.rs:3:10
   |
3  | #[derive(Copy, Clone)]
   |          ^^^^ conflicting implementation for `SpookyThing<_>`
...
16 | impl<T> Copy for SpookyThing<T> {}
   | ------------------------------- first implementation here

error[E0119]: conflicting implementations of trait `std::clone::Clone` for type `SpookyThing<_>`:
 --> src/main.rs:3:16
  |
3 | #[derive(Copy, Clone)]
  |                ^^^^^ conflicting implementation for `SpookyThing<_>`
...
8 | impl<T> Clone for SpookyThing<T> {
  | -------------------------------- first implementation here
