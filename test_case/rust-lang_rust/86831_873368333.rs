rust
let total_size_usize = group_size_usize
    .checked_mul(info.groups.len())
    .ok_or_else(host_memory_space_overlow)?;
