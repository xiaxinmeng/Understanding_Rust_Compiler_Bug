 rust
use std::sync::{Arc, Mutex};
use std::thread;

#[derive(PartialEq, Eq)]
pub struct Value {
    x: i32
}
impl Value {
    pub fn new(x: i32) -> Value { Value { x: x } }

    pub fn increment(&mut self) { self.x += 1; }
}

fn main() {
    let p = Arc::new(Mutex::new(Value::new(0)));

    for _ in 1..100 {
        let x = p.clone();
        thread::spawn(move || {
            let n;
            {
                let mut x_ = x.lock().unwrap();
                x_.increment();
                n = *x_;
            }
            thread::sleep_ms(50);
            {
                // because Rust guarantees thread-safety, we can safely assume x hasn't been changed
                let x_ = x.lock().unwrap();
                if *x_ != n {
                    panic!();
                }
            }
        });
    }

    thread::sleep_ms(2000);
}
