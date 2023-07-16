rust
struct V<const U: usize = {1+2}>
where
    [(); U]:;
