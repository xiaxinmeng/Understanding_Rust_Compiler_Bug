rust
macro_rules! thread_local {
    ($(#[$attr:meta])* $vis:vis static $name:ident: $t:ty = const { $init:expr }) => (
        // ...
    );

    ($(#[$attr:meta])* $vis:vis static $name:ident: $t:ty = $init:expr) => (
        // ...
    );

    // ...
}
