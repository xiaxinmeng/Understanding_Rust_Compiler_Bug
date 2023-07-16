Rust
#[bench]
pub fn bench_typed_arena_clear_100(b: &mut Bencher) {
    let mut arena = TypedArena::default();
    b.iter(|| {
        for _ in 0..100 {
            arena.alloc(Point { x: 1, y: 2, z: 3 });
        }
        arena.clear();
    })
}
