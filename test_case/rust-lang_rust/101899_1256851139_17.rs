rust
  // debug-assertions = false
  use slabmalloc::{AllocablePage, Rawlink, SCAllocator};
  use std::{alloc::Layout, sync::atomic::AtomicU64};
  struct DummyPage;
  impl AllocablePage for DummyPage {
      const SIZE: usize = usize::MAX - 46;
      fn bitfield(&self) -> &[AtomicU64; 8] { unimplemented!() }
      fn bitfield_mut(&mut self) -> &mut [AtomicU64; 8] { unimplemented!() }
      fn prev(&mut self) -> &mut Rawlink<Self> { unimplemented!() }
      fn next(&mut self) -> &mut Rawlink<Self> { unimplemented!() }
  }
  let mut alloc = SCAllocator::<DummyPage>::new(usize::MAX - 126);
  let layout = Layout::from_size_align(0, 128).unwrap();
  alloc.allocate(layout).unwrap();
  // calls Layout::from_size_align_unchecked(usize::MAX - 126, 128)
  // at slabmalloc::sc::SCAllocator::<'_, DummyPage>::allocate()
  