
#[bench]
fn bench_copy_4MB(b: &mut Bencher) {
    let mut file = test::black_box(File::open("4MB").unwrap());
    let mut writer: Vec<u8> = Vec::with_capacity(4 * 1024 * 1024);

    b.iter(|| {
        let mut x = Ok(0);
        for _ in 0..100 { x = io::copy(&mut file, &mut writer); writer.clear(); }
        x
    })
}
