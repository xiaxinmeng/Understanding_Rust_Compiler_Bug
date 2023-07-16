
error[E0271]: type mismatch resolving `<[closure@src/main.rs:10:12: 10:19] as std::ops::FnOnce<()>>::Output == u8`
  --> src/main.rs:10:5
   |
5  | fn higher<T: F8>(t: T) -> u8 {
   |    ------    -- required by this bound in `higher`
...
10 |     higher(|| 3u16);
   |     ^^^^^^ expected `u8`, found `u16`
