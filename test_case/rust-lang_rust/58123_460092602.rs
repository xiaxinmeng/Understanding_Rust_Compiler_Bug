rust
use std::collections::BinaryHeap;

#[no_mangle]
pub extern fn is_opt(heap: &mut BinaryHeap<u32>) -> bool {
    if let Some(mut peek) = heap.peek_mut() {
        *peek = 1;
        true
    } else {
        false
    }
}
