 rust
#[inline(never)]
fn writer_write(w: &mut Writer, b: &[u8]) {
    w.write(b);
}

#[bench]
fn bench_write_virt(bh: &mut BenchHarness) {
    bh.iter(|| {
        let mut mem = MemWriter::new();
        let wr = &mut mem as &mut Writer;
        for _ in range(0, 1000) {
            writer_write(wr, "abc".as_bytes());
        }
    });
}
