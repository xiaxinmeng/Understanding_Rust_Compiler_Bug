
use std::i32;
use std::mem::forget;
use std::sync::Arc;

fn main() {
    let three = Arc::new(3);
    for _ in 0..i32::MAX {
        let x = three.clone();
        forget(x);
    }
    println!("done");
}
