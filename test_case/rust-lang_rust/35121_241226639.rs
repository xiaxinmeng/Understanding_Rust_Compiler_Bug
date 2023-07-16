 Rust
trait Balls: Default {}
impl Balls for () {}

struct Flah;

impl Flah {
    fn flah<T: Balls>(&self) -> T {
        Default::default()
    }
}

fn doit(cond: bool) {
    let _ = if cond {
        Flah.flah()
    } else {
        return
    };
}

fn main() {
    let _ = doit(true);
}
