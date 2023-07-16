rust
#[bench]
fn x(b: &mut Bencher) {
    let mut v = Vec::with_capacity(100);
    b.iter(|| {
        for n in 0..100 {
            v.push(n);
        }
        v.truncate(0);
    }
    );
}
