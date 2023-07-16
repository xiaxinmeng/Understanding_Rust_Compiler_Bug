rust
> use std::convert::TryFrom;
> 
> assert_eq!(usize::try_from(7_u64).unwrap(), 7_usize);
> 