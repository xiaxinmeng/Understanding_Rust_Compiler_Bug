rust
  fn reinterstruct(ptr_pair: Box<PairFoo>) -> Box<[Foo]> {
    let mut ptr_pair = ptr_pair;
    unsafe {
        Vec::from_raw_parts(&mut ptr_pair.fst as *mut Foo, 2, 2)
            .into_boxed_slice()
    }
    // ptr_pair is dropped here, freeing the memory. 
    // Returned Box<[Foo]> refers to the freed memory.
}
