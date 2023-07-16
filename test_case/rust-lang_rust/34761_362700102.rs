rust
unsafe impl <#[may_dangle] T, A: Allocator> Drop for AllocVec<T, A> {
  fn drop(&mut AllocVec<T, A>) {
    // can rely on A being alive, but not T
  }
}
