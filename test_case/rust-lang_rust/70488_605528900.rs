
fn bench_ready(c: &mut Criterion) {
    let mut group = c.benchmark_group("future::ready");

    group.bench_function("futures", |b| {
        b.iter(|| {
            executor::block_on(async {
                black_box(futures::future::ready(42)).await;
            })
        })
    });
