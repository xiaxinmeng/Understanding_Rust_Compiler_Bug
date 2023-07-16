rust
#![feature(fmt_as_str)]

extern "C" {
    fn write_str(_: &str);
}

fn write_fmt(args: core::fmt::Arguments) {
    if let Some(s) = args.as_str() {
        // Avoid allocation, write the &str directly.
        unsafe { write_str(s) };
    } else {
        // Format into a new String to get a &str.
        unsafe { write_str(&args.to_string()) };
    }
}

macro_rules! write {
    ($($t:tt)*) => {
        write_fmt(format_args!($($t)*))
    };
}

pub fn a() {
    write!("hello world");
}
