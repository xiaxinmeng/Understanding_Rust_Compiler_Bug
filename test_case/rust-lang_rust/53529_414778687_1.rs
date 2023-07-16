rust
use std::collections::VecDeque;

fn main() {
    let mut dst = VecDeque::new();
    dst.push_front(12);
    dst.push_front(1234);
    dst.pop_back();

    let mut src = VecDeque::new();
    src.push_front(1);
    dst.append(&mut src);
    for (i, a) in dst.iter().enumerate() {
        println!("{}, {}", i, a);
    }
}

