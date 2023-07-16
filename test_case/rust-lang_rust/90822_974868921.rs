rust
let mut an_allocator = Allocator::new();
an_allocator.with_sub_allocator(|sub_allocator| {
    let pinned = Box::pin_in(42, sub_allocator);
    mem::forget(pinned);
});
