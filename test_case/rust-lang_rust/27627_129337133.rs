
if cap == 0 { 
    let new_cap = cmp::max(64 / mem::size_of::<T>(),  1);
    alloc(new_cap);
} else { 
    let new_cap = if pretty_small_or_pretty_big::<T>(cap) { // see FBVector source for details
        cap * 2
    } else { 
        // No need to check match because we're not `pretty_big` (< 1 MB)
        (cap * 3 + 1) / 2 
    };
    realloc(cap, new_cap);
}
