rust
#![allow(incomplete_features, unused_variables, dead_code)]
#![feature(const_fn, const_trait_impl)]

use std::ops::Add;

#[derive(Copy, Clone)]
struct NotConstAdd {}

impl Add for NotConstAdd {
    type Output = Self;

    fn add(self, _other: Self) -> Self::Output {
        self
    }
}

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

fn main() {
    // Both of these work, because they're not being called in a const context.
    do_add(NotConstAdd {});
    really_do_add(NotConstAdd {});
}
