rust
#[bench]
fn starts_with(b: &mut Bencher) {
    let text = black_box("kdjsfhlakfhlsghlkvcnljknfqiunvcijqenwodind\0");
    b.iter(|| {
        for i in 0..1024 {
            black_box(text.starts_with('k'));
        }
    })
}

#[bench]
fn contains(b: &mut Bencher) {
    let text = black_box("kdjsfhlakfhlsghlkvcnljknfqiunvcijqenwodind\0");
    b.iter(|| {
        for i in 0..1024 {
            black_box(text.contains('\0'));
        }
    })
}
