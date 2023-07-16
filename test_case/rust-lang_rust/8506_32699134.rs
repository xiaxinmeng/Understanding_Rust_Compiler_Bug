 rust
#[allow(dead_code)];

enum Either {
    One,
    Other(~int, int)
}

static one : Either = One;

fn main () {}
