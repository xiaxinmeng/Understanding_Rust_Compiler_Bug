rust
if cmp == Equal { Ok(base) } else { Err(base + (cmp == Less) as usize) }
