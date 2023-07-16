 rust
let lock = ..;
let _l1 = lock.lock();
let _l2 = lock.lock();
println!("oops!");
