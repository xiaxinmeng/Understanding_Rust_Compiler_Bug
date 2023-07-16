rust
  // overflow-checks = false
  use hecs::World;
  World::new().reserve::<([u8; usize::MAX / 262144 + 1],)>(262144);
  // calls alloc::alloc() with size 0
  // at hecs::archetype::Archetype::grow_exact()
  