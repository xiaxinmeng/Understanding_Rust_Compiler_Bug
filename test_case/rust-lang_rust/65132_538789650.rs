rust
use std::collections::VecDeque;

pub fn vecdeque_with_exact_capacity<T>(capacity: usize) -> Result<VecDeque<T>, ()> {
    let vd = VecDeque::with_capacity(capacity);
    if vd.capacity() != capacity {
        return Err(());
    }
    Ok(vd)
}

pub fn main() -> Result<(), ()> {
    let size = std::env::args().next().unwrap().parse().unwrap();
    let mut v = vecdeque_with_exact_capacity::<u8>(size)?;
    v.push_front(1);
    println!("{:?}", v);
    Ok(())
}
