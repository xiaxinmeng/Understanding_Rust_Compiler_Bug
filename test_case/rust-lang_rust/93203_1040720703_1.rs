rust
scope(|s| {
    s.spawn(|| {
        //  ^^ look ma, no `s`
        s.spawn(|| { ... });
        //      ^^ look ma, no `s`
    })
})
