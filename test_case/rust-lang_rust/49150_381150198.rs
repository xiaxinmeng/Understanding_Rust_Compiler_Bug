rust
macro_rules! pin_fields {
    ($pin:expr, ($($field:ident),+ $(,)?)) => {
        unsafe {
            let s = Pin::get_mut(&mut $pin);
            ($(Pin::new_unchecked(&mut s.$field),)+)
        }
    };
}
