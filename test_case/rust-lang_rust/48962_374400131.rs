rust
#![feature(nll)]

fn main() {
    let mut src = &mut (22, 44);
    {src};
    src.0 = 66;
}
