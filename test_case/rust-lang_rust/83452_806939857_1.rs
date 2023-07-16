rust
const fn do_add<T: Copy + Add<Output = T>>(a: T) -> T {
    // Don't use Add
    a
}

const fn other_fn() -> bool {
    false
}

const fn really_do_add<T: Copy + Add<Output = T>>(a: T) -> T {
    let cond = other_fn();
    // Do use Add, just in non-obviously dead code.
    if cond { a + a; }
    a
}

// Works.
const A: NotConstAdd = do_add(NotConstAdd {});
// Also works.
const B: NotConstAdd = really_do_add(NotConstAdd {});
