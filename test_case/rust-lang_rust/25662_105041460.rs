 rust
#![feature(catch_panic, collections, std_misc)]
use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::collections::BTreeSet;
use std::thread;
use std::sync::{Arc, Mutex};

#[derive(PartialEq, Eq, Ord, Debug, Clone)]
struct Panicker<T>(T);

impl<T: PartialEq> PartialOrd for Panicker<T>
{
    fn partial_cmp(&self, _: &Self) -> Option<Ordering>
    {
        panic!()
    }
}

fn main() {
    let heap = Arc::new(Mutex::new(BinaryHeap::new()));
    let heap_ref = heap.clone();
    let guard = thread::catch_panic(move || {
        let mut local_heap = heap.lock().unwrap();
        local_heap.push(Panicker(BTreeSet::<i32>::new()));
        local_heap.push(Panicker(BTreeSet::<i32>::new()));
    });
    println!("Thread result: {:?}", guard);

    let mutex_guard = match heap_ref.lock() {
        Ok(inner) => inner,
        Err(e) => e.into_inner(),
    };

    let vector_from_binary_heap = mutex_guard.clone().into_vec();
    println!("{:?}", vector_from_binary_heap);
}
