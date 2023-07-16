rust
 // If we eliminate the last row, any left-over inputs are considered missing
 if i >= mat.len() {
    return Some(Issue::Missing(i));   // Fix to Issue::Missing(0)
 }
