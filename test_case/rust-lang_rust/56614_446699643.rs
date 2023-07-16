rust
let a = AtomicCell::new(vec![]);

// Thread #1
a.store(vec![1, 2, 3], Relaxed);

// Thread #2
println!("{:?}", a.take(Relaxed)); // unsynchronized read!
