Rust
fn subslice_pattern_test(a: &Allocator, arg: bool, arg2: bool) {
    let a = [a.alloc(), a.alloc(), a.alloc(), a.alloc(), a.alloc()];
    if arg2 {
        drop(a);
        return;
    }

    if arg {
        let[.., _x, _] = a;
    } else {
        let[_, _y..] = a;
    }
}
