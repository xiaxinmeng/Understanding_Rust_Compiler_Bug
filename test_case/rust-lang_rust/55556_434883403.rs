
  │     Total:     17,813,640 bytes (8.23%) in 66 blocks (0.01%), avg size 269,903.64 bytes, avg lifetime 1,210,149,468.11 instrs (75.11% of program duration)
  │     At t-gmax: 12,861,916 bytes (30.3%) in 58 blocks (0.05%), avg size 221,757.17 bytes
  │     At t-end:  0 bytes (0%) in 0 blocks (0%), avg size 0 bytes
  │     Reads:     4,774,894 bytes (1.11%), avg 0.27 per byte
  │     Writes:    17,813,574 bytes (6.87%), avg 1 per byte
  │     Location: {
  │       #1: 0x6E271D4: alloc (alloc.rs:78)
  │       #2: 0x6E271D4: alloc (alloc.rs:154)
  │       #3: 0x6E271D4: allocate_in<u8,alloc::alloc::Global> (raw_vec.rs:106)
  │       #4: 0x6E271D4: with_capacity<u8> (raw_vec.rs:150)
  │       #5: 0x6E271D4: with_capacity<u8> (vec.rs:368)
  │       #6: 0x6E271D4: read<&std::path::Path> (fs.rs:267)
  │       #7: 0x6E271D4: get_metadata_section_imp (locator.rs:895)
  │       #8: 0x6E271D4: rustc_metadata::locator::get_metadata_section (locator.rs:852)
  │       #9: 0x6E257ED: rustc_metadata::locator::Context::extract_one (locator.rs:615)
  │     }
