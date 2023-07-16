rust
#[bench]
fn current(b: &mut Bencher) {
    let mut v = Vec::with_capacity(512);
    let mut r = 0u32;
    v.clear();
    for _ in 0..512 {
        r = r.wrapping_mul(1664525).wrapping_add(1013904223);
        v.push(r);
    }

    b.iter(|| {
        let v = black_box(&v);
        let mut p = v.as_ptr();
        let end = v.as_ptr().wrapping_offset(v.len() as isize);
        let mut sum = 0;
        while p != end {
            sum += unsafe { *p };
            p = p.wrapping_offset(1);
        }
        black_box(sum)
    });
}
