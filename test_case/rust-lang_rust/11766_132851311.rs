
coreyf@frewbook-air /tmp> cat hello-world.rs
#![feature(start)]

#[test]
fn test() {
    assert!(false);
}

#[start]
fn start(_argc: isize, _argv: *const *const u8) -> isize {
    0
}
