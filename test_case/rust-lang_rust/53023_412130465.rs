
    unsafe {
        let [left_lo, left_hi] = std::mem::transmute::<u128, [u64; 2]>(left);
        let [right_lo, right_hi] = std::mem::transmute::<u128, [u64; 2]>(right);
