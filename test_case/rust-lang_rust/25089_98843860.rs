 rust
#[test]
#[should_panic]
fn panic() {
    should_panic(Box::new(1), empty());
}

struct Empty;

fn empty() -> Empty { Empty }

#[inline(never)]
fn should_panic(_: Box<u32>, _: Empty) {
    panic!("test panic");
}
