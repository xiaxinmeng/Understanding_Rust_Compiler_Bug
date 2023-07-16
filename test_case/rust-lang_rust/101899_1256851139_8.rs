rust
  use v_frame::plane::PlaneData;
  PlaneData::<u8>::new(0);
  // calls alloc::alloc() with size 0
  // at v_frame::plane::PlaneData::<u8>::new_uninitialized()
  