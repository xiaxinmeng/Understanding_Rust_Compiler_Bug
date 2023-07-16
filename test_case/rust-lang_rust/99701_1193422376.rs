rust
use std::collections::VecDeque;

fn main() {
    let mut tester: VecDeque<usize> = VecDeque::with_capacity(3);

    for i in 0..3 {
        tester.push_back(i);
    }

    let _: VecDeque<_> = tester.drain(1..2).collect();
}
