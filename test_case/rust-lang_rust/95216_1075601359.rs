rs
    criterion.bench_function("1", |bencher| {
        bencher.iter(|| {
            let url = "https://google.com".to_owned();
            let split_specifier = url.as_str().split(':');
            split_specifier.skip(1).collect::<String>();
        })
    });

    criterion.bench_function("2", |bencher| {
        bencher.iter(|| {
            let url = "https://google.com".to_owned();
            let split_specifier = url.as_str().split(':');
            split_specifier.skip(1).collect::<Vec<_>>().join("");
        })
    });
  