
/// Reserves the minimum capacity for the given `BitvSet` to contain `len` distinct elements.
/// In the case of `BitvSet` this means reallocations will not occur as long as all inserted
/// elements are less than `len`.
///
/// Note that the allocator may give the collection more space than it requests. Therefore
/// capacity can not be relied upon to be precisely minimal. Prefer `reserve_len` if future
/// insertions are expected.
