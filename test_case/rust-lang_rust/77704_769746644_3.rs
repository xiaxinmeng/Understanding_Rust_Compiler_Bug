rust
let subslice = slice::checked_range(bounds, ..slice.len()).and_then(|range| &bytes::get(range));
