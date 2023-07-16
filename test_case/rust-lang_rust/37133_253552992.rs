 rust
let x = Mutex::new(5);
let mut y = x.lock();
let mut z = x.lock(); //2 mutable pointers
