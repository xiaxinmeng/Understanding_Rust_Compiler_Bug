rust
use std::thread::ScopedJoinHandle;

fn x<'a>(_: &'a i32, h: ScopedJoinHandle<'a, i32>) -> ScopedJoinHandle<'a, i32> {
    h
}

fn main() {
    std::thread::scope(|s| {
        let mut v = Vec::new();
        {
            let a = 1;
            v.push(x(&a, s.spawn(|| 1)));
        }
    });
}
