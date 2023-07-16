 rust
// this is a TreeSet<int, CacheAlignedAllocator>
let mut xs = TreeSet::with_alloc(CACHE_ALIGNED_ALLOCATOR);
xs.insert(5);
channel.send(xs);
