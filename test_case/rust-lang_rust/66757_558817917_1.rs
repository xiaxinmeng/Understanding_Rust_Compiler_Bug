
error[E0277]: `?` couldn't convert the error to `F`
  --> src/lib.rs:20:24
   |
20 |     u32::try_from(1u32)?;
   |                        ^ the trait `std::convert::From<()>` is not implemented for `F`
   |
   = note: the question mark operation (`?`) implicitly performs a conversion on the error value using the `From` trait
   = help: the following implementations were found:
             <F as std::convert::From<E>>
   = note: required by `std::convert::From::from`
