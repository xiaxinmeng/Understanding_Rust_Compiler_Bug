 rust
use std::thread::Thread;
use std::sync::{Arc,Mutex};

fn main() {
    let numbers = Arc::new(Mutex::new(vec![1, 2, 3]));

    for i in range(0, 3) {
        let number = numbers.clone();
        Thread::spawn(|&:| {
            let i = 5;
            let mut array = number.lock().unwrap();
            (*array)[i] += 1;
            println!("numbers[{}] is {}", i, (*array)[i]);
        });
    }
}
