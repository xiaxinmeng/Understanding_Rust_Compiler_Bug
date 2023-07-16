sh
  # --target x86_64-unknown-linux-gnu
  scryer-prolog < /dev/null
  # calls alloc::realloc() with prior size 18432 on an allocation of size 36864
  # at scryer_prolog::machine::raw_block::RawBlock::<scryer_prolog::machine::heap::StandardHeapTraits>::grow()
  