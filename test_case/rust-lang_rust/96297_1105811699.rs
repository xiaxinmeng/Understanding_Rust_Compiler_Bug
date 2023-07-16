rust
> let mut maybe_result: Vec<u16> = s.encode_wide().chain([0]).collect(); 
> if unrolled_find_u16s(0, &maybe_result[..maybe_result.len() - 1]).is_some() {
> 