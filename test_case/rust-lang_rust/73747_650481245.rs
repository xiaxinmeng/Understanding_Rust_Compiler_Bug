rust
unsafe {
    ptr::swap(
        ts.gc_arenas_full as *mut _ as *mut HashSet<*mut u8>,
        &mut ts.condemned_arenas,
    )
}
