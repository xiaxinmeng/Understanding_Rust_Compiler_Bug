rust
const fn do_add<T: Copy + Add<Output = T>>(a: T) -> T {
    // Don't use Add
    a
}

const fn really_do_add<T: Copy + Add<Output = T>>(a: T) -> T {
    // Do use Add
    a + a
}

// Works.
const A: NotConstAdd = do_add(NotConstAdd {});

// Does not work.
const B: NotConstAdd = really_do_add(NotConstAdd {});
