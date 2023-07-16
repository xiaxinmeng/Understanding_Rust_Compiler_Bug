rust
use std::sync::Mutex;

fn main() {
    let lock = Mutex::new(vec![1; 10]);
    let mut inner = lock.lock().unwrap();
    while let Some(x) = inner.pop() {
        if x != 1 {
            break;
        }
        inner.push(x + 1);
        println!("the lock works as expected");
    }
}
