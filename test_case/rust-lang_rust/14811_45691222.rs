 rust
let x = RefCell::new(1);
let y = x.borrow_mut();

// this will fail, due to the `y` mutable borrow stopping
// any other borrow/borrow_mut.
println!("{}", x);
