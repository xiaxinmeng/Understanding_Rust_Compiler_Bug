rust
pub fn test(n: usize) {
    let sum = Mutex::new(0);
    std::thread::scope(|s| {
        for i in 0..n {
            s.spawn(|| {
                *sum.lock().unwrap() += i;
            });
        }
    });
}
