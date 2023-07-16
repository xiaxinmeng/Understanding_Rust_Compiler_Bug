rust
macro_rules! is_ident {
    ($i:ident) => {};
}

macro_rules! check {
    () => {
        is_ident!($crate);
    };
}

check!();
