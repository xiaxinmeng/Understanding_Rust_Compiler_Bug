compile_fail,E0277
> let x = 7_u64;
> // error: `std::convert::From<u32>` is not implemented for `u16`
> let _ = u32::from(x); // (What if `x` was `7_000_000_000_000_u64` ?)
> 