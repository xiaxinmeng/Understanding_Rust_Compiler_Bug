 rust
extern crate revord;

let mut pq = BinaryHeap::new();

pq.push((RevOrd(1), "foo"));
pq.push((RevOrd(0), "bar"));
pq.push((RevOrd(2), "baz"));

assert_eq!(pq.pop(), (RevOrd(0), "bar"));
