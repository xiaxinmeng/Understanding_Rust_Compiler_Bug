rust
macro_rules! a {
    ($e:expr) => {
        match ($e,) {
            (e,) => &[e],
        }
    };
}

macro_rules! b {
    ($e:expr) => {
        &[$e]
    };
}

fn main() {
    let _: &'static [i32] = a!(1); // fail
    let _: &'static [i32] = b!(1); // ok
}
