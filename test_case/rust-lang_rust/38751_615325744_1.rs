
error[E0277]: `?` couldn't convert the error to `E`
 --> src/lib.rs:9:13
  |
9 |     Ok(f.e()?)
  |             ^ the trait `std::convert::From<<impl Fallible as Fallible>::Error>` is not implemented for `E`
  |
  = note: the question mark operation (`?`) implicitly performs a conversion on the error value using the `From` trait
  = note: required by `std::convert::From::from`
