rust
macro_rules! make_println {
    ($d:tt $name:ident, $fmt:expr) => {
        macro_rules! $name {
            ($d($d args:expr),*) => {           // (1)
                println!($fmt, $d($d args),*);  // (2)
            }
        }
    };
}

make_println!($ dbg, "{:?}");
