rust
macro_rules! make_error {
    (...) => {
        #[must_use]
        {
            let error = ...;
            error
        }
    };
}
