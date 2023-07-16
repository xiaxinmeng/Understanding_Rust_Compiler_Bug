rust
fn bar() {
    let lock: RwLock<HashMap<u32, String>> = RwLock::new(HashMap::new());
    
    if let Some(item) = lock.read().unwrap().get(&5) {
        println!("its in there")
    } else { lock.write().unwrap().entry(5).or_insert(" eggs".to_string());
        println!("ok we put it there")
    }
}
