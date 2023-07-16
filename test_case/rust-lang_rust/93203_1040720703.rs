rust
scope(|s| {
    s.spawn(|s| {
        s.spawn(|s| { ... });
    })
})
