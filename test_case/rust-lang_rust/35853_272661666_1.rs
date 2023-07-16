rust
macro_rules! make_println {
    ($name:ident, $fmt:expr) => {
        macro_rules! $name {
            ($$($$args:expr),*) => {           // (1)
                println!($fmt, $$($$args),*);  // (2)
            }
        }
    };
}
