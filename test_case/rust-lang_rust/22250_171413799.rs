 rust
macro_rules! foo {
    () => {
        #[cfg(feature = "nightly")]
        const fn bar() {}
        #[cfg(not(feature = "nightly"))]
        fn bar() {}
    };
}
foo!();
