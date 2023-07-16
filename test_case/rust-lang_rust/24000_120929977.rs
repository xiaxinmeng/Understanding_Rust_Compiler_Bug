 rust
let runs: Vec<Range<usize>> = /* ... */;
let mut iter = runs.iter().flat_map(Clone::clone);
