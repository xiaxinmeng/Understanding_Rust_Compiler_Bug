rust
let res = mutex.with(|inner| undulate(inner)).unwrap();
// ... vs ...
let res = mutex.map(|inner| undulate(&mut *inner).unwrap();
