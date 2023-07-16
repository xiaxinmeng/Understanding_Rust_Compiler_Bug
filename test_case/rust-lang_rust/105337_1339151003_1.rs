rust
error[[E0631]](https://doc.rust-lang.org/stable/error-index.html#E0631): type mismatch in function arguments
  --> src/main.rs:18:14
   |
18 |         .map(<&mut [u8]>::from)
   |          --- ^^^^^^^^^^^^^^^^^
   |          |   |
   |          |   expected due to this
   |          |   found signature defined here
   |          required by a bound introduced by this call
   |
   = note: expected function signature `fn(&mut Vec<u8>) -> _`
              found function signature `fn(&mut [u8]) -> _`
note: required by a bound in `map`
