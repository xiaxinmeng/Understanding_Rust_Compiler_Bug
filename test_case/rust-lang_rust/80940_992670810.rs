rust
mod x {
    macro_rules! private {
        () => {};
    }

    #[macro_export]
    macro_rules! public {
        () => {
            private!();
        };
    }
}

crate::public!();
