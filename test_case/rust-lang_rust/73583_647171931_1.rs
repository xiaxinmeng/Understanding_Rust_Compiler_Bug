rust
    // src/module.rs

    use std::panic::Location;

    pub fn get() -> &'static Location<'static> {
        f()
    }

    #[track_caller]
    fn f() -> &'static Location<'static> {
        Location::caller()
    }
    