rust
fn workload(buf: &[u8]) {
    let mut des = Deserializer::new(buf);
    for _ in 0..400 {
        match des.deserialize_bytes::<2>() {
            Ok(ok) => {
                let _ = black_box(ok);
            }
            Err(err) => {
                let _ = black_box(err);
            }
        }
    }
}

#[bench]
fn test_playground(b: &mut Bencher) {
    let buf: [u8; 1024] = [0; 1024];
    b.iter(|| workload(black_box(&buf)));
}
