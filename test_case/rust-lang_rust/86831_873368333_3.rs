rust
let Some(total_size_usize) = group_size_usize
    .checked_mul(info.groups.len())
else { host_memory_space_overlow() };
