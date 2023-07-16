rust
#[test_case]
#[cfg(target_runner = quickcheck::run)]
fn identity(a: i32) -> bool { ... }

#[test_case]
#[cfg(target_runner = criterion::bench)]
fn benchmark(c: &mut Criterion) { ... }
