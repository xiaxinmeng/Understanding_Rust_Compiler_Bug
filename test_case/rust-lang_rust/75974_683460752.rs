rust
let mut max = heap.peek_mut().unwrap();
if x < *max {
    *max = x;
}
