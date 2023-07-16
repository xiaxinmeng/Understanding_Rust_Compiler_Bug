
    /// loop-block:
    ///    can_go = index_or_cur == length_or_end
    ///    if can_go then succ else drop-block
    /// drop-block:
    ///    if size_of::<T> != 0 {
    ///        ptr = index_or_cur
    ///        index_or_cur = index_or_cur + size_of::<T>()
    ///    } else {
    ///        ptr = &mut LV[index_or_cur]
    ///        index_or_cur = index_or_cur + 1
    ///    }
    ///    drop(ptr);
