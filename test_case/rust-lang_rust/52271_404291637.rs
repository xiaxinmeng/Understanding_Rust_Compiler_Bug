rust
// Returns a Layout which describes the allocation required for a hash table,
// and the offset of control bytes in the allocation.
fn calculate_layout<K, V>(capacity: usize) -> Result<(Layout, usize), LayoutErr> {
    // Array of key-value pairs
    let pairs = Layout::array::<(K, V)>(capacity)?;

    // Array of control bytes, with duplicated control bytes at the end of the
    // array so that we don't need bounds checking while scanning. These mirror
    // the control bytes at the front of the array
    let num_ctrl_bytes = capacity.checked_add(MATCH_GROUP_SIZE - 1).ok_or(LayoutErr)?;
    let ctrl = Layout::array::<u8>(num_ctrl_bytes)?;

    pairs.extend(ctrl)
}

