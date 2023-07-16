 rust
let mut iter = pattern.split('*');
match (iter.next(), iter.next(), iter.next()) {
    // exactly 2 elements
    (Some(head), Some(tail), None) => { ... }
    _ => {}
}
