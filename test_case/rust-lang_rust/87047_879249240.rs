rust
> let (evens, odds) = (0..10).map(|n| if n % 2 == 0 { Ok(n) } else { Err(n) }).collect();
> 