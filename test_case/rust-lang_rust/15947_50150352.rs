
#[deriving(InvertedOrd)]
struct Key<T>(T);

let mut pq = PriorityQueue::new();
pq.push((Key(2), 10u));
