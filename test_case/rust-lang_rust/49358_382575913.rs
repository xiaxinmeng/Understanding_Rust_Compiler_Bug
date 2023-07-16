rust
let arc = ManuallyDrop::new(Arc::from_raw(ptr)).clone();
