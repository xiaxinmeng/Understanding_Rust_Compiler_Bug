rust
> let (a_len, b_len) = (a.len(), b.len());
> assert!(a_len == b_len);
> return (0..a_len).map(|_| (a.next().unwrap(), b.next().unwrap()))
> 