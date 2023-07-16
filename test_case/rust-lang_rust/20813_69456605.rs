 rust
let x = Cell::new(1);

Thread::scoped(move || {
    x.set(2);
}).join();

println!("{}", x.get());
