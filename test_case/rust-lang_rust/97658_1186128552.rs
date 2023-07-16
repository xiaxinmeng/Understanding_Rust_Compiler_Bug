rust
fn print_args(args: fmt::Arguments) {
   /* */
}

macro_rules! println {
    ($fmt:expr) => ($crate::print_args(format_args_nl!($fmt)));
    ($fmt:expr, $($arg:tt)*) => ($crate::print_args(format_args_nl!($fmt, $($arg)*)));
}

fn main() {
    let number = 1;
    println!("The number is {number}");
}
