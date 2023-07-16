rust
// Nightly, Edition 2018 on.

#![feature(uniform_paths)]

// Let's assume that there's a crate `foo` in the extern prelude with `foobar` in it.

mod foo {
    pub fn foobar() -> u8 { 1 }
}

mod bar {
    fn barfoo() -> u8 {
        // With the current `uniform_paths`,
        // there's no conflict as far as the resolution system sees it.
        //
        // Thus, if `foo` is in the extern prelude, it will resolve to the
        // `foo` crate and not `foo` the sibling module. If we allow that
        // to pass, then a *breaking change* will be the result if we change
        // to the file centric behavior.
        foo::foobar()
    }
}
