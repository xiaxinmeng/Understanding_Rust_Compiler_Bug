rust
// Don't care about extra
for c in a.exact_chunks(16) {}

// Shouldn't have extra
let mut chunks = a.exact_chunks(16); 
assert!(chunks.extra().is_empty(), "Had extra things");
for c in &mut chunks {}

// Handle extra
let mut chunks = a.exact_chunks(16); 
for c in &mut chunks {}
for b in chunks.extra() {}
