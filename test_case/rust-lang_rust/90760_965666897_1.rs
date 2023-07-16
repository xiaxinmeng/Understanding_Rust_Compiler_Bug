rust
fn do_few_actions_on_mutexes(m: &Mutex<String>){
    let reference = &*m.lock().unwrap();
    // If we extend lifetime as you suggested,
    // lock should be held here
    // but the fact that we keep mutex locked is surprising for reader.
    *reference = "Hello world".to_owned();
}
