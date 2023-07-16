
    executor::block_on(async {
        let mut group = c.benchmark_group("future::ready");

        group.bench_function("futures", |b| {
            b.iter(move || async {
                black_box(futures::future::ready(42)).await
            })
        });
