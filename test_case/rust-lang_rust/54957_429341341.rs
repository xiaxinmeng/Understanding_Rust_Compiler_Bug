rust
    fn as_key_slice(&self) -> &'a [K] {
        // When taking a pointer to the keys, if our key has a stricter
        // alignment requirement than the shared root does, then the pointer
        // would be out of bounds, which LLVM assumes will not happen. If the
        // alignment is more strict, we need to make an empty slice that doesn't
        // use an out of bounds pointer.
        if mem::align_of::<K>() > mem::align_of::<LeafPrefix>() && self.is_shared_root() {
            &[]
        } else {
            // Here either it's not the root, or the alignment is less strict,
            // in which case the keys pointer will point "one-past-the-end" of
            // the node, which is allowed by LLVM.
            unsafe {
                slice::from_raw_parts(
                    &(*self.node as *const LeafNode<K, V>).keys as *const K,
                    self.len()
                )
            }
        }
    }
