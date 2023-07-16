 rust
macro_rules! attr_block {
    ($(#[$attrs:meta])* {}) => {};

    (
        $(#[$attrs:meta])* {
            $it:item $($tail:tt)*
        }
    ) => {
        $(#[$attrs])*
        $it

        attr_block! { $(#[$attrs])* {$($tail)*} }
    };
}
