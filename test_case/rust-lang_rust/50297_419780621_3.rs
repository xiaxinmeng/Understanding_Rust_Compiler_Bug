rust
#[test_case(quickcheck::run)]
fn identity(a: i32) -> bool { ... }

#[test_case(criterion::bench)]
fn benchmark(c: &mut Criterion) { ... }
