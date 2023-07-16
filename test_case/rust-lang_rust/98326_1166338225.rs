rust
#[bench]
fn bench_next_chunk(b: &mut Bencher) {
    let v = vec![13u8; 2048];

    b.iter(|| {
        const CHUNK: usize = 8;

        let mut sum = [0u32; CHUNK];
        let mut iter = black_box(v.clone()).into_iter();

        while let Ok(chunk) = iter.next_chunk::<CHUNK>() {
            for i in 0..CHUNK {
                sum[i] += chunk[i] as u32;
            }
        }

        sum
    })
}
