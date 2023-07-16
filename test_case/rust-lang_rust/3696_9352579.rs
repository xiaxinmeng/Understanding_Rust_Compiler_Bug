
fn a(blk: &a/fn(c: &blk/fn() -> &blk/fn())) {
    let d = || { ... };
    do blk {
        return d;
    }
}
