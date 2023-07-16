
...
/// This collection is wasteful in time and space during incr-full builds,
/// because for those, all nodes are new. However, the waste is relatively
/// small, and the maintenance cost of avoiding using this for incr-full
/// builds is somewhat high and prone to bugginess. It does not seem worth
/// it at the time of this writing, but we may want to revisit the idea.
hybrid_indices: IndexVec<DepNodeIndex, CompressedHybridIndex>,
