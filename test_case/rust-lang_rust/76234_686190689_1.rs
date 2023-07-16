rust
// instead of into_iter_sorted
for item in heap.into_sorted() {
    ..
}

// into_sorted_vec stays around
let v = heap.into_sorted_vec();

// instead of drain_sorted
for item in heap.sorted_mut().drain() {
    ..
}

// like drain_sorted, but doesn't clear the heap
let max_5 = heap.sorted_mut().into_iter().take(5).collect::<Vec<_>>();
