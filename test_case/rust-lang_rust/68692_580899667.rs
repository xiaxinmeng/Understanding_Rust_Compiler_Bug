
2020-01-31T20:25:18.5690130Z    Compiling alloc v0.0.0 (/checkout/src/liballoc)
2020-01-31T20:25:21.2480774Z error[E0308]: mismatched types
2020-01-31T20:25:21.2481479Z     --> src/liballoc/vec.rs:2399:9
2020-01-31T20:25:21.2481892Z      |
2020-01-31T20:25:21.2482209Z 2398 |     fn from(arr: [T; N]) -> Self {
2020-01-31T20:25:21.2482609Z      |                             ---- expected `vec::Vec<T>` because of return type
2020-01-31T20:25:21.2482957Z 2399 |         <[T]>::into_vec(box arr)
2020-01-31T20:25:21.2483368Z      |         ^^^^^^^^^^^^^^^^^^^^^^^^ expected struct `vec::Vec`, found struct `std::vec::Vec`
2020-01-31T20:25:21.2483955Z      = note: expected struct `vec::Vec<T>`
2020-01-31T20:25:21.2484259Z                 found struct `std::vec::Vec<T>`
