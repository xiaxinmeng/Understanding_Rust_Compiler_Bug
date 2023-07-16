rust
use std::collections::VecDeque;

fn ab(dq: &mut VecDeque<i32>, sz: usize) {
    for i in 0..sz {
        dq.push_back(i as _);
    }
    dq.make_contiguous();
    for _ in 0..sz {
        dq.pop_front();
    }
}

fn main() {
    let mut dq = VecDeque::with_capacity(2);
    ab(&mut dq, 2);
    ab(&mut dq, 3);
    
    dbg!(dq.len()); // this is zero
    
    dbg!(dq.pop_front()); // uaf+double frees
}
