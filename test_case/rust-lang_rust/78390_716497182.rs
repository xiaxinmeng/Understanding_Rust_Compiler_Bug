rust
use std::sync::Mutex;

fn main() {
    let lock = Mutex::new(vec![1; 1]);
    while let Some(x) = lock.lock().unwrap().pop() {
        if x != 1 {
            break;
        }
        lock.lock().unwrap().push(x + 1);
        println!("the lock works as expected");
    }
}
