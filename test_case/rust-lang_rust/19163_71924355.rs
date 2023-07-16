 rust
// mywrite.rs
#![crate_type="lib"]

#[macro_export]
macro_rules! mywrite {
    ($dst:expr, $($arg:tt)*) => ({
        let dst = &mut *$dst;  //~ error: cannot borrow immutable borrowed content as mutable
        (|&mut: args| { dst.write_fmt(args) })(format_args!($($arg)*))
    })
}
