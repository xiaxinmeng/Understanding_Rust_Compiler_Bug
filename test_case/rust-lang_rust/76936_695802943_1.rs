rust
unsafe {
    // SAFETY: within this scope there are no other references to `x`'s contents,
    // so ours is effectively unique.
    let p1_exclusive: &mut i32 = p1.assume_unique()
    *p1_exclusive += 27;
}

unsafe {
    // SAFETY: within this scope nobody expects to have exclusive access to `x`'s contents,
    // so we can have multiple shared accesses concurrently.
    let p2_shared: &i32 = p2.assume_no_muts();
    assert_eq!(*p2_shared, 42 + 27);
    let p1_shared: &i32 = p1.assume_no_muts();
    assert_eq!(*p1_shared, *p2_shared);
}
