
error: the type of this value must be known in this context
 --> test.rs:6:29
  |
6 |         .filter(|_, (a, b)| *a == b'\r' && *b == b'\n')
  |                             ^^

error[E0599]: no method named `map` found for type `std::iter::Filter<std::iter::Enumerate<std::iter::Zip<std::slice::Iter<'_, u8>, std::iter::Skip<std::slice::Iter<'_, u8>>>>, [closure@test.rs:6:17: 6:55]>` in the current scope
 --> test.rs:7:10
  |
7 |         .map(|(i, (_, _))| i);
  |          ^^^
  |
  = note: the method `map` exists but the following trait bounds were not satisfied:
          `[closure@test.rs:6:17: 6:55] : std::ops::FnMut<(&(usize, (&_, &_)),)>`
          `std::iter::Filter<std::iter::Enumerate<std::iter::Zip<std::slice::Iter<'_, u8>, std::iter::Skip<std::slice::Iter<'_, u8>>>>, [closure@test.rs:6:17: 6:55]> : std::iter::Iterator`

error[E0593]: closure takes 2 arguments but 1 argument is required
 --> test.rs:6:10
  |
6 |         .filter(|_, (a, b)| *a == b'\r' && *b == b'\n')
  |          ^^^^^^ -------------------------------------- takes 2 arguments
  |          |
  |          expected closure that takes 1 argument

error[E0593]: closure takes 2 arguments but 1 argument is required
 --> test.rs:6:10
  |
6 |         .filter(|_, (a, b)| *a == b'\r' && *b == b'\n')
  |          ^^^^^^ -------------------------------------- takes 2 arguments
  |          |
  |          expected closure that takes 1 argument

error: aborting due to previous error(s)

