rust
fn test_pooled_emit() {
    let mut emitter = PooledEventEmitter::new();

    emitter.add_listener("test", box || {
        thread::sleep_ms(1000);
        println!("Thread: {}", thread_id::get());
        Ok(())
    }).unwrap();

    emitter.add_listener_value::<i32, _>("test", box |arg| {
        println!("Thread: {}, {:?}", thread_id::get(), arg);
        Ok(())
    }).unwrap();

    emitter.add_listener_value::<&str, _>("test", box |arg| {
        println!("Thread: {}, {:?}", thread_id::get(), arg);
        Ok(())
    }).unwrap();

    let a = box emitter.emit("test");
    let b = box emitter.emit_value("test", 10);
    let c = box emitter.emit_value("test", "test");

    let all = box a.join3(b, c);

    assert_eq!(all.wait().unwrap(), (3, 3, 3));
}
