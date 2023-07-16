rust
macro_rules! with_dollar_sign {
    ($($body:tt)*) => {
        macro_rules! __with_dollar_sign { $($body)* }
        __with_dollar_sign!($);
    }
}

macro_rules! make_println {
    ($name:ident, $fmt:expr) => {
        with_dollar_sign! {
            ($d:tt) => {
                macro_rules! $name {
                    ($d($d args:expr),*) => {           // (1)
                        println!($fmt, $d($d args),*);  // (2)
                    }
                }
            }
        }
    };
}

make_println!(my_dbg, "{:?}");

fn main() {
    my_dbg!(42);
}
