rust
macro_rules! my_option {
    ($t:path) => {
        union MyOption {
            data: *const dyn $t,
            raw_bytes: [usize; 2]
        }
        ...
    }
}
