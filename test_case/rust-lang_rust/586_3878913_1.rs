
fn alloc_vec(size: uint) -> [int]: is_not_empty(*) { ... }

fn f() -> int {
    let xs = alloc_vec(10u);    // is_not_empty(xs) is a post-condition
    ret vec::head(xs);          // is_not_empty(xs) is a pre-condition
}
