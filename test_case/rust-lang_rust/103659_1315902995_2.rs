rs
    c.bench_function("EE", |b| b.iter(|| {
        let (l, r) = black_box((FE::A(12), FE::B));
        black_box(l.partial_cmp(&r));
    }));

    c.bench_function("FE", |b| b.iter(|| {
        let (l, r) = black_box((EE::A, EE::B));
        black_box(l.partial_cmp(&r));
    }));

    c.bench_function("EF", |b| b.iter(|| {
        let (l, r) = black_box((EF::A, EF::B(12)));
        black_box(l.partial_cmp(&r));
    }));

    c.bench_function("FEF", |b| b.iter(|| {
        let (l, r) = black_box((FEF::A(24), FEF::C(56)));
        black_box(l.partial_cmp(&r));
    }));

    c.bench_function("EFE", |b| b.iter(|| {
        let (l, r) = black_box((EFE::<usize>::A, EFE::<usize>::C));
        black_box(l.partial_cmp(&r));
    }));

    c.bench_function("FEEF", |b| b.iter(|| {
        let (l, r) = black_box((FEEF::A(500), FEEF::D(10)));
        black_box(l.partial_cmp(&r));
    }));

    c.bench_function("EFFE", |b| b.iter(|| {
        let (l, r) = black_box((EFFE::<usize>::A, EFFE::<usize>::D));
        black_box(l.partial_cmp(&r));
    }));

    c.bench_function("EFFEFFF", |b| b.iter(|| {
        let (l, r) = black_box((EFFEFFF::<usize>::A, EFFEFFF::G(10)));
        black_box(l.partial_cmp(&r));
    }));
