rust
let alloc = MyAlloc(/* ... */);
let pinned = Box::pin_in(42, alloc);
mem::forget(pinned); // Now `value` must live forever
// Otherwise `Pin`'s invariants are violated, storage invalidated
// before Drop was called.
// borrow of `memory` can end here, there is no value keeping it.
drop(alloc); // Oh, value doesn't live forever.
