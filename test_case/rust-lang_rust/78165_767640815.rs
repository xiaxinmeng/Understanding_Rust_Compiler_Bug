rust
let runtest = Arc::new(Mutex::new(Some(move || match opts.strategy {
    …
})));
…
cfg.spawn({
    let runtest = Arc::clone(&runtest);
    move || runtest.lock().unwrap().take().unwrap()()
})
…
runtest.lock().unwrap().take().unwrap()()
