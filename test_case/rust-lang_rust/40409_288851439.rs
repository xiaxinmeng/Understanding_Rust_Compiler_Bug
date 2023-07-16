rust
#[bench]
fn bench_bufreader(b: &mut Bencher) {
    b.iter(|| {
        let f = File::open("calloc.rs").unwrap();
        let b = BufReader::new(f);
        b.bytes().count()
    });
}
