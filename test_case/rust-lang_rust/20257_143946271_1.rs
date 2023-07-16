 rust
let cell = ArcCell::new(Arc::new(1));

thread::scoped(|| { // (or equivalent)
     let a = cell.swap(Arc::new(2), Ordering::SeqCst);
     assert_eq!(*a, 1);
     // no other references, so `a` is destroyed here
})
