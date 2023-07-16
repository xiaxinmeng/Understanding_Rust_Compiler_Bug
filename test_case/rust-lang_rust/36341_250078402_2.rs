 rust
let join_handle = thread::spawn(|| {
    thread::current()
});

let dead_thread = join_handle.join().unwrap();
println!("Thread name: {:?}", dead_thread.name());
assert!(thread::current().id() != dead_thread.id());
