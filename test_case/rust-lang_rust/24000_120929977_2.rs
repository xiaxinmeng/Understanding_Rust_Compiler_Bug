 rust
let runs: Vec<Range<usize>> = /* ... */;
fn id(x: Range<usize>) -> Range<usize> { x }
let mut iter = runs.iter().cloned().flat_map(id as fn(x: Range<usize>) -> Range<usize>);
