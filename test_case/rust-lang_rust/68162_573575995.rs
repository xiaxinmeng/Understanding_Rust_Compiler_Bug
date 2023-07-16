rust
fn one(_: u32) -> u32 {
    1
}

fn main() {
    static SOME_STATIC: Option<_> = unimplemented!();
    SOME_STATIC.map(one);
}
