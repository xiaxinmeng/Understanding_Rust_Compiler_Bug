rust
use std::iter::{self, FusedIterator};
use std::sync::mpsc::{channel, Sender};

fn main() {
    let (sender, receiver) = channel();
    let iter = iter::from_fn(|| receiver.try_recv().ok());
    for i in 1..=10 {
        sender.send(i).unwrap();
    }
    run(&sender, iter.cycle());
}

fn run(sender: &Sender<i32>, mut iter: impl FusedIterator<Item = i32>) {
    for x in &mut iter {
        dbg!(x);
    }
    sender.send(42).unwrap();
    assert_eq!(iter.next(), None);
}
