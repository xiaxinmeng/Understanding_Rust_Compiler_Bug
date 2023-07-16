 rust
// If the type `root::bar` is `root::foo::bar -> root`, then `pub(root::bar)` is public and the glob is shadowed.
// If the type `root::bar` is `root::foo`, then `pub(root::bar)` is private to `foo` and the glob is not shadowed.
mod root {
    mod glob_imported {
        pub use root::foo as bar;
    }
    mod foo {
        pub fn bar() {}
        pub(root::bar) use root as bar;
    }
    use root::glob_imported::*;
    use root::foo::bar;
}

// The same thing happens here, except that the bindings are not stable in either
// configuration instead of being stable in both configurations.
mod root {
    mod glob_imported {
        pub use root as bar;
    }
    mod foo {
        pub fn bar() {}
        pub(root::bar) use root::foo as bar;
    }
    use root::glob_imported::*;
    use root::foo::bar;
}
