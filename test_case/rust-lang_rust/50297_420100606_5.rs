rust
fn identity(a: i32) -> bool { ... }

#[test_case]
const identity_test: SimpleTest = SimpleTest {
   test_fn: || { quickcheck_run(identity) },
   test_name: "identity",
   is_bench: false
}

fn benchmark(c: &mut Criterion) { ... }

#[test_case]
const benchmark_test: CriterionTest = CriterionTest {
   bench_fn: benchmark,
   test_name: "benchmark"
}
