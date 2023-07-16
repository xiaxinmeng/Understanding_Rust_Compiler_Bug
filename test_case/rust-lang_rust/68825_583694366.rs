rust
 181             match usize::from_str_radix(s, 10) {
 182                 Ok(0) => Ok(None),
                     // this would be nicer as `Ok(i) => Ok(Some(i.try_into().unwrap()))`
 183                 Ok(i) => Ok(Some(NonZeroUsize::new(i).unwrap())),
 184                 Err(e) => Err(e),
 185             }
