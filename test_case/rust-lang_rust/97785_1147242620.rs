rust
use std::collections::LinkedList;

fn insert<T>(list: &mut LinkedList<T>, element: T, at: usize) {
    let mut tail = list.split_off(at);
    list.push_back(element);
    list.append(&mut tail);
}
