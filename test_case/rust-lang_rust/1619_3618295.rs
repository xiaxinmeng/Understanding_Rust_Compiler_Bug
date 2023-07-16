
fn iter<A>(v: [A], blk: fn(A) -> loop_ctl) {
    let i = 0u, n = vec::len(v);
    while i < n {
        alt blk(v[i]) {
            lc_break { ret; }
            lc_cont { /* fallthrough */ }
        }
    }
}
