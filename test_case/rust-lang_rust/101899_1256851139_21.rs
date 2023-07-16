rust
  // overflow-checks = false
  use ring_buffer::RingBuffer;
  RingBuffer::<u16>::with_capacity(usize::MAX / 4 + 2);
  // calls alloc::alloc() with size 0
  // at ring_buffer::RingBuffer::<u16>::with_capacity()
  