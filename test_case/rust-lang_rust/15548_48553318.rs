 rust
#[cfg(not(ndebug))]
macro_rules! debug_show {
    ($($it: item)*) => {
        $(
          #[deriving(Show)] $it
        )*
    }
}
#[cfg(not(ndebug))]
macro_rules! debug_show {
    ($($it: item)*) => {
        $( $it )*
    }
}

debug_show! {
    struct Foo { ... }
    enum Bar { ... }
}
