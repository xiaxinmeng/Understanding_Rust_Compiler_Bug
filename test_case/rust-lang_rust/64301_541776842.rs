rust
let mut buffer = std::mem::Uninitialized::<Foo, N>::uninit();
let queue = SmallSmallVec::new(&mut buffer);
