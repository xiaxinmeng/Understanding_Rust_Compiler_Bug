 rust
    let foo = Arc::new(Foo::new());
    let foo_ = foo.clone();

    panic::catch_unwind(move || foo.bar());
    thread::spawn(move || foo_.bar()).join();
