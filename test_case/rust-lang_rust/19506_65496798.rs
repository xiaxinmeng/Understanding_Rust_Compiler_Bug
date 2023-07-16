 rust
#![feature(macro_rules)]

macro_rules! format_lite {
    ($($piece:expr)#+ $(,$arg:expr)+) => {
        ::std::fmt::format(&::std::fmt::Arguments::new(&[$($piece),*],
            &[$(::std::fmt::argument(::std::fmt::Show::fmt, &$arg)),*]))
    }
}

fn main() {
    std::io::println(&*format_lite!("foo"#"bar", 5u));
}
