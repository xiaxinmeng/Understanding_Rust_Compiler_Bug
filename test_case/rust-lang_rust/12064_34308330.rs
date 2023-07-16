 rust
#[unsafe_destructor]
impl<K, V, IS> Drop for NodeRef<K, V, IS> {
    fn drop(&mut self) {
        unsafe {
            let node: &mut UnsafeNode<K, V, IS> = cast::transmute(self.ptr);
            let old_count = node.ref_count.fetch_sub(1, Acquire);
            assert!(old_count >= 1);
            if old_count == 1 {
                node.destroy()
            }
        }
    }
}
