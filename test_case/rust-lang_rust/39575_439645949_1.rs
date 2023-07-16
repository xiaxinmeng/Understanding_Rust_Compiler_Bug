rust
fn foo(led: Led) {
    let m1 = Arc::new(Mutex::new(Some(led)));
    let m2 = Arc::clone(&w);

    Command::new("true")
        .before_exec(move || Ok(m2.lock().unwrap().take().unwrap().on()))
        .status().unwrap();
    w.lock().unwrap().take().unwrap().off();
}
