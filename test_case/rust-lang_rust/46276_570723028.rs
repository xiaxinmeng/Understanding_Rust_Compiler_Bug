rust
use std::sync::Mutex;

fn main() {
    let m = Mutex::new(42);
    
    let x = m.lock().unwrap();

    // println!("{}", *x - 1);
    println!("{}", x - 1);
    
    /*
    error[E0369]: cannot substract `{integer}` from `std::sync::MutexGuard<'_, {integer}>`
 --> src/main.rs:9:22
  |
9 |     println!("{}", x - 1);
  |                    - ^ - {integer}
  |                    |
  |                    std::sync::MutexGuard<'_, {integer}>
  |
  = note: an implementation of `std::ops::Sub` might be missing for `std::sync::MutexGuard<'_, {integer}>`
    */
}
