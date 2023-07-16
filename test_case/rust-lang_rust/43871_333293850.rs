
error[E0532]: expected tuple struct/variant, found enum `Result`
  --> src/lib.rs:16:12
   |
16 |     if let Result(y) = x {
   |            ^^^^^^
   |
   = note: did you mean to use one of the following variants?
           - `std::result::Result::Ok`
           - `std::result::Result::Err`
