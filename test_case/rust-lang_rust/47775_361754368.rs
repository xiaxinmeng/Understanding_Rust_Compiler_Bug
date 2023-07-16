rust
macro_rules! name_maybe_fn_from_ident {
    ( @as_expr, $e:expr) => { $e };
    ( @generate_fn, $name:tt) => {
        fn name_maybe<'a>() -> Option<&'a str> {
            Some(name_maybe_fn_from_ident!( @as_expr, $name ))
        }
    };
    ( $name:ident ) => { name_maybe_fn_from_ident!( @generate_fn, (stringify!($name))); }
}

name_maybe_fn_from_ident!(rah);

fn main() {}
