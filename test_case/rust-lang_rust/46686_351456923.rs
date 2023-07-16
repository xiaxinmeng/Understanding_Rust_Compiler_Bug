Rust
fn subslice_pattern_test(a: &Allocator, arg: bool) {
    let a = [a.alloc(), a.alloc(), a.alloc(), a.alloc(), a.alloc()];
    if arg {
        let[.., _x, _] = a;
    } else {
        let[_, _y..] = a;
    }
}
