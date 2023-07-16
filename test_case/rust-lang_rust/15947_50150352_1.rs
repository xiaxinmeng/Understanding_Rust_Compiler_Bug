
struct InvertOrd<T>(T);

// Not sure how this would work?
impl<T> Ord for InvertOrd<T> { ... }

let mut pq = PriorityQueue::new();
pq.push((InvertOrd(2i), 10u));
