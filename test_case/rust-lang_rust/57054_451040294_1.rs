
error[E0308]: mismatched types

error[E0308]: mismatched types
help: you can cast an `u32` to `f64`, producing the floating point representation of the integer
  |
1 | .into()#![feature(try_from)]
  | ^^^^^^^

error[E0277]: cannot subtract `euclid::size::TypedSize2D<f64, MapUnit>` from `euclid::point::TypedPoint2D<f64, MapUnit>`
  |
  = help: the trait `std::ops::Sub<euclid::size::TypedSize2D<f64, MapUnit>>` is not implemented for `euclid::point::TypedPoint2D<f64, MapUnit>`

error[E0308]: mismatched types
  |
  = note: expected type `&str`
             found type `std::string::String`

error[E0308]: mismatched types
  |
  = note: expected type `&tile::Tile`
             found type `tile::Tile`

error: aborting due to 5 previous errors
