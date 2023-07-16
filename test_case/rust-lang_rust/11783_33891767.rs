 rust
#[cfg(test)]
mod rust {
    #[bench]
    fn iterbytes(bench: &mut BenchHarness) {
        use std::to_bytes::{IterBytes};
        bench_base( bench, |v| {
            let mut xxh: XXHState = XXHState::new(0);
            v.iter_bytes(true, |bytes| {
                xxh.update(bytes);
                true
            });
            xxh.digest();
        });
    }

    #[bench]
    fn oneshot(bench: &mut BenchHarness) {
        bench_base( bench, |v| {
            xxh32(v, 0);
        });
    }
}

#[cfg(test)]
mod siphash {
    #[bench]
    fn oneshot(bench: &mut BenchHarness) {
        use std::hash;
        use std::hash::{Streaming};
        bench_base( bench, |v| {
            let mut sip: hash::State = hash::default_state();
            sip.write(v);
            sip.result_u64();
        });
    }

    #[bench]
    fn iterbytes(bench: &mut BenchHarness) {
        bench_base( bench, |v| {
            v.hash();
        });
    }
}
