rust
  // overflow-checks = false
  use assembler::{ExecutableAnonymousMemoryMap, InstructionStreamHints};
  use std::mem;
  let mut map = ExecutableAnonymousMemoryMap::new(0, false, false).unwrap();
  map.instruction_stream(&InstructionStreamHints {
      number_of_labels: usize::MAX / mem::size_of::<usize>() + 1,
      ..Default::default()
  });
  // calls alloc::alloc() with size 0
  // at assembler::LabelledLocations::new()
  