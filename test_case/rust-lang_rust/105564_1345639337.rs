rs

#[bench]
fn bench_vec_deque_to_vec(b: &mut Bencher) {
    let mut ring = VecDeque::from(vec![0; 1024]);
    ring.shrink_to_fit();
    assert_eq!(ring.len(), ring.capacity());

    b.iter(move || {
        assert_eq!(ring.capacity(), 1024);
        unsafe {
            ring.set_head(512);
            // slow path #1: second branch of `make_contiguous` (`tail <= free`)
            ring.set_len(768);
        }

        let v = Vec::from(std::mem::take(&mut ring));
        ring = VecDeque::from(v);

        assert_eq!(ring.capacity(), 1024);

        unsafe {
            ring.set_head(512);
            // slow path #2: third branch of `make_contiguous` (`head_len > tail_len`)
            ring.set_len(769);
        }

        let v = Vec::from(std::mem::take(&mut ring));
        ring = VecDeque::from(v);
    })
}
