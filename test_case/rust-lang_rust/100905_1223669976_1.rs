rust
struct W(usize);

impl AddAssign<W> for usize {
    fn add_assign(&mut self, rhs: W) {
        *self += rhs.0;
    }
}

pub fn test2(n: usize) {
    let sum = Mutex::new(0);
    std::thread::scope(|s| {
        for i in (0..n).map(W) {
            s.spawn(|| {
                *sum.lock().unwrap() += i;
            });
        }
    });
}
