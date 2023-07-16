
extern crate test;

use std::io::MemWriter;
use test::Bencher;

#[bench]
fn bench_write_value(bh: &mut Bencher) {
    bh.iter(|| {
        let mut mem = MemWriter::new();
        for _ in range(0u, 1000) {
            mem.write("abc".as_bytes());
        }
    });
}

#[bench]
fn bench_write_ref(bh: &mut Bencher) {
    bh.iter(|| {
        let mut mem = MemWriter::new();
        let wr = &mut mem as &mut Writer;
        for _ in range(0u, 1000) {
            wr.write("abc".as_bytes());
        }
    });
}

#[bench]
fn bench_write_macro1(bh: &mut Bencher) {
    bh.iter(|| {
        let mut mem = MemWriter::new();
        let wr = &mut mem as &mut Writer;
        for _ in range(0u, 1000) {
            write!(wr, "abc");
        }
    });
}

#[bench]
fn bench_write_macro2(bh: &mut Bencher) {
    bh.iter(|| {
        let mut mem = MemWriter::new();
        let wr = &mut mem as &mut Writer;
        for _ in range(0u, 1000) {
            write!(wr, "{}", "abc");
        }
    });
}
