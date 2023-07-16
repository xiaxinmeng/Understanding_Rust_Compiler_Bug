rust
    let size = mem::size_of::<T>()
        .checked_mul(count)
        .expect("is_nonoverlapping: size_of::<T>() * count overflows a usize");
