rust
// Type invariant: constructing an element of this type will cause a heap allocation
pub struct Evil(());

impl Evil {
  pub const fn new() {
    drop(Box::new(0));
    Evil
  }

  pub check(&self) {
    let number_of_heap_allocs = /* call private allocator API */;
    if number_of_heap_allocs == 0 { unsafe { unreachable_unchecked() }}
  }
}
