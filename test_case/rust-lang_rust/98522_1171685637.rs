
let mut heap_vec = heap.into_vec();
heap_vec.iter_mut().for_each(update_entry);
heap = BinaryHeap::from(heap_vec);
