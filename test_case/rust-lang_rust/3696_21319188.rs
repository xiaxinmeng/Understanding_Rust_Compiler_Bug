 rust
fn foo<'a>(blk: &fn(p: &'a fn() -> &'a fn())) {
    let mut state = 0;
    let statep = &mut state;
    do blk {
        || { *statep = 1; }
    }
}
fn main() {
    do foo |p| { p()() }
}
