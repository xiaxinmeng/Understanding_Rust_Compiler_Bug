rust
/// One-indexed, because the first CloneCounter is included. If you don't
/// want the original to count, construct a [``CloneCounterObserver`]
/// instead and use [`CloneCounterObserver::counter`] to increment.
pub struct CloneCounter;

pub struct CloneCounterObserver;
