
/tmp $ cat hi.rs
fn main() {
    let x = 4;
    let y = 10;
    let z = 20;

    assert!(x + y > z);
}

/tmp $ rustc hi.rs; and ./hi
thread '<main>' panicked at 'assertion failed: x + y > z', hi.rs:6

/tmp $ rustc -vV
rustc 1.0.0-nightly (2b01a37ec 2015-02-21) (built 2015-02-21)
binary: rustc
commit-hash: 2b01a37ec38db9301239f0c0abcf3c695055b0ff
commit-date: 2015-02-21
build-date: 2015-02-21
host: x86_64-apple-darwin
release: 1.0.0-nightly
