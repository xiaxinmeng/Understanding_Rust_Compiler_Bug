rust
if let Some(mut peek) = heap.peek_mut() {
    let popped = std::mem::replace(&mut peek, pushed);
    Some(popped)
} else {
    None
}
