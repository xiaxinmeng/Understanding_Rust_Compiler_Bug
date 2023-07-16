rust
#[quickcheck]
fn identity(a: i32) -> bool { ... }

#[criterion_bench]
fn benchmark(c: &mut Criterion) { ... }
