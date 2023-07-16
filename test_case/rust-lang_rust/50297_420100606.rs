rust
#![cfg_attr(quickcheck, test_runner(quickcheck::runner))]
#![cfg_attr(criterion, test_runner(criterion::runner))]


#[cfg(quickcheck)]
mod quickcheck_tests {
    #[quickcheck]
    fn identity(a: i32) -> bool { ... }
}

#[cfg(criterion)]
mod criterion_benches {
    #[criterion_bench]
    fn benchmark(c: &mut Criterion) { ... }
}
