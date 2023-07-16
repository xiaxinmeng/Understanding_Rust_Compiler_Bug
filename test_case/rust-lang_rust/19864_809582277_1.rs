
#[bench]
fn b2(b: &mut test::Bencher) {
    b.iter(|| {
        let r: Result<u8, Bar> = Ok(1u8);
        iter::repeat_with(||r.as_ref()).take(N).map(|x| test::black_box(x)).count()
    });
}
