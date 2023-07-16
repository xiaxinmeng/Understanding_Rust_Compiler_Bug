 rust
extern mod extra;
use extra::test::BenchHarness;

#[bench]
fn fmt(b: &mut BenchHarness) {
    do b.iter {
        do 100.times {
            fmt!("aasd f df %d awe", 4);
        }
    }
}

#[bench]
fn ifmt(b: &mut BenchHarness) {
    do b.iter {
        do 100.times {
            ifmt!("aasd f df {:d} awe", 4);
        }
    }
}
