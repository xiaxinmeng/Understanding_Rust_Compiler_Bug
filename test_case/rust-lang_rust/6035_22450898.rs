 rust
extern mod extra;

#[inline(never)]
fn used(_: ~str) {}

#[bench]
fn vanilla(b: &mut extra::test::BenchHarness) {
    do b.iter {
        let a = ~"test";
        used(a)
    }
}
#[bench]
fn ifmt_no_args(b: &mut extra::test::BenchHarness) {
    do b.iter {
        let a = ifmt!("test");
        used(a)
    }
}
#[bench]
fn ifmt_one_arg(b: &mut extra::test::BenchHarness) {
    do b.iter {
        let a = ifmt!("{:s}", "test");
        used(a)
    }
}
