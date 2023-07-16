rust
macro_rules! my_struct {
    ($(#[$meta:meta])* $ident:ident) => {
        $(#[$meta])* struct $ident;
    }
}

my_struct!(#[derive(Debug)] Foo);
