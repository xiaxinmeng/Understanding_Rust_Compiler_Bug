
error[E0061]: this function takes 1 parameter but 2 parameters were supplied
 --> src/main.rs:8:5
  |
8 |     Ok(s, 0)
  |     ^^^^^^^^ expected 1 parameter

error[E0308]: mismatched types
 --> src/main.rs:8:5
  |
8 |     Ok(s, 0)
  |     ^^^^^^^^ expected enum `Res`, found enum `std::result::Result`
  |
  = note: expected enum `Res<'a, T>`
             found enum `std::result::Result<_, _>`

error[E0061]: this function takes 1 parameter but 0 parameters were supplied
  --> src/main.rs:14:28
   |
7  | fn get_res<'a, T>(s: &'a str) -> Res<'a, T> {
   | ------------------------------------------- defined here
...
14 |     println!("{}", is_err!(get_res()));
   |                            ^^^^^^^^^ expected 1 parameter

error[E0308]: mismatched types
  --> src/main.rs:11:48
   |
11 | macro_rules! is_err { ($e:expr) => (match $e { Err(_) => true, Ok(_, _) => false})}
   |                                                ^^^^^^ expected enum `Res`, found enum `std::result::Result`
...
14 |     println!("{}", is_err!(get_res()));
   |                    ------------------
   |                    |       |
   |                    |       this match expression has type `Res<'_, _>`
   |                    in this macro invocation
   |
   = note: expected enum `Res<'_, _>`
              found enum `std::result::Result<_, _>`

error[E0308]: mismatched types
  --> src/main.rs:11:64
   |
11 | macro_rules! is_err { ($e:expr) => (match $e { Err(_) => true, Ok(_, _) => false})}
   |                                                                ^^^^^^^^ expected enum `Res`, found enum `std::result::Result`
...
14 |     println!("{}", is_err!(get_res()));
   |                    ------------------
   |                    |       |
   |                    |       this match expression has type `Res<'_, _>`
   |                    in this macro invocation
   |
   = note: expected enum `Res<'_, _>`
              found enum `std::result::Result<_, _>`

error[E0023]: this pattern has 2 fields, but the corresponding tuple variant has 1 field
   --> src/main.rs:11:64
    |
11  | macro_rules! is_err { ($e:expr) => (match $e { Err(_) => true, Ok(_, _) => false})}
    |                                                                ^^^^^^^^ expected 1 field, found 2
...
14  |     println!("{}", is_err!(get_res()));
    |                    ------------------ in this macro invocation

error: aborting due to 6 previous errors
