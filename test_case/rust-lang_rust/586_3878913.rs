
fn f() -> int {
    let xs = alloc_vec(10u);
    check is_not_empty(xs);
    ret vec::head(xs);
}
