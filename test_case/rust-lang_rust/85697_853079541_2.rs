rust
// ...
    // SAFETY: This call is safe because `append` does not create
    // aliases to already existing elements in `v`.
    unsafe {
        append(v);
    }
// ...
