rust
// These two yield the same items.
iter.map_while(predicate)
iter.scan((), |(), item| predicate(item)).fuse()
