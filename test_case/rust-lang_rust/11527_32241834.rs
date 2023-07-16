 rust
/// Return a new `BoundedPriorityQueue` instance, holding at most `maximum` elements.
fn new(maximum: uint) -> BoundedPriorityQueue<A>;

/// Pop the largest value from the queue, blocking until the queue is not empty
fn pop(&self) -> A;

/// Pop the largest value from the queue, or return `None` if the queue is empty.
fn try_pop(&self) -> Option<A>;

/// Pop the largest value from the queue, blocking until the queue is not empty or the timeout
/// expires.
fn pop_timeout(&self, reltime: Time) -> Option<A>;

/// Push a value into the queue, blocking until the queue is not full.
fn push(&self, item: A);

/// Push a value into the queue, or return `Some(item)` if the queue is full.
fn try_push(&self, item: A) -> Option<A>;

/// Push a value into the queue, blocking until the queue is not full or the timeout expires. If
/// the timeout expires, return `Some(item)`.
fn push_timeout(&self, item: A, reltime: Time) -> Option<A>;
