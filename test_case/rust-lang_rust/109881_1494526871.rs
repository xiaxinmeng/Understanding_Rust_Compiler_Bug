rs
macro_rules! bad {
    ($pat:path) => {
        #[allow($pat)] {}
    };
}
