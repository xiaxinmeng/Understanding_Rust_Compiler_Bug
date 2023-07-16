compile_fail,E0277
> let x = 7_u64;
> // error: `std::convert::From<u64>` is not implemented for `usize`
> let _ = usize::from(x);
> 