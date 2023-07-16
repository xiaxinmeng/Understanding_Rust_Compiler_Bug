
running 3 tests
test roundtrip           ... bench:          13 ns/iter (+/- 0)
test roundtrip_w_nallocx ... bench:          15 ns/iter (+/- 0)
test roundtrip_w_sallocx ... bench:          19 ns/iter (+/- 1)



#[bench]
fn roundtrip_w_nallocx(b: &mut Bencher) {
    b.iter(|| unsafe {
        let ptr = jemalloc_sys::mallocx(100, 0);
        let rsz = jemalloc_sys::nallocx(100, 0);
        jemalloc_sys::sdallocx(ptr, rsz, 0);
        // rsz
    });
}

#[bench]
fn roundtrip_w_sallocx(b: &mut Bencher) {
    b.iter(|| unsafe {
        let ptr = jemalloc_sys::mallocx(100, 0);
        let rsz = sallocx(ptr as usize, 0);
        jemalloc_sys::sdallocx(ptr, rsz, 0);
        // rsz
    });
}

#[bench]
fn roundtrip(b: &mut Bencher) {
    b.iter(|| unsafe {
        let ptr = jemalloc_sys::mallocx(100, 0);
        // let rsz = sallocx(ptr as usize, 0);
        jemalloc_sys::sdallocx(ptr, 100, 0);
    });
}
