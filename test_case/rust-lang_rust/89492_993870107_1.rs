rust
u32vec.iter().fold(0, |acc, x| acc + x.abs_diff(median))        // as per experimental API for u32
