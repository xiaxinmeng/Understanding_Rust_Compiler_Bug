rust
fn calculate_layout<K, V>(capacity: usize) -> Result<(Layout, usize), LayoutErr> {
    let pairs = Layout::array::<(K, V)>(capacity)?;
    
    let ctrl = Layout::array::<u8>(capacity)?;
    let dup_ctrl = Layout::array::<u8>(MATCH_GROUP_SIZE - 1)?;
    let (ctrl, offset) = ctrl.extend(dup_ctrl)?;
    debug_assert_eq!(offset, capacity);
    
    pairs.extend(ctrl)
}
