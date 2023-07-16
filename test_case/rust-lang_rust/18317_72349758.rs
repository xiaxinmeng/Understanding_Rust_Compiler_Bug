 rust
macro_rules! defn {
    ($n:ident) => (
        fn $n (&self) -> i32 {
            println!("{}", stringify!($n));
            1
        }
        )
}

#[derive(Copy)]
pub struct S;

impl S {
    // Attempt to define a public method via a macro.
    // The resulting method ends up being non-public.
    pub defn!(f);
}
