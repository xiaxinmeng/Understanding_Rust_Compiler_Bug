rust
fn f1() {}

fn f2() {}

struct S {}

impl From<fn()> for S {
    fn from(f: fn()) -> S {
        S {}
    }
}

fn main() {
    // Example code did this, which works:
    let s1 = S::from(if true { f1 } else { f2 });
    // My code did this, which does not work:
    let s2 = S::from(f1);
    // I also tried this, which also does not work:
    let s3 = S::from(if true { f1 } else { f1 });
}
