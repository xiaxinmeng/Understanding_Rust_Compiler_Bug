rust
> #![feature(non_portable_conversion)]
> 
> # // Make this test a no-op on non-64-bit platforms
> # #[cfg(target_pointer_width = "64")]
> use std::target::PointerWidthGe64From;
> 
> # #[cfg(target_pointer_width = "64")]
> assert_eq!(usize::target_from(7_u64), 7_usize);
> 