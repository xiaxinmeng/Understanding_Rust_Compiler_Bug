rs
fn criterion_benchmark(criterion: &mut Criterion) {
    criterion.bench_function("1", |bencher| {
        bencher.iter(|| {
            vec!["hello", "world"]
                .iter()
                .map(|item| item.to_uppercase())
                .collect::<Vec<String>>()
                .join("")
        })
    });
    criterion.bench_function("2", |bencher| {
        bencher.iter(|| {
            vec!["hello", "world"]
                .iter()
                .map(|item| item.to_uppercase())
                .collect::<String>()
        })
    });
}
