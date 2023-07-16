rust
let v = vec![];
scope(|s| {
    s.spawn(move || {
        for e in &v {
        }
    });
    s.spawn(move || {
        for e in &v {
        }
    });
})
