rust
// panics if r isn't a subset of ..self.len()
let Range { start, end } = slice::range_from_subset(r, ..self.len());
