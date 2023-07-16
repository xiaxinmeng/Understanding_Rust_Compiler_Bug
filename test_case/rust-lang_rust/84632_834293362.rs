rust
#![recursion_limit = "5"]
macro_rules! a {
    () => ("");
    (A) => (concat!("", a!()));
    (A, $($A:ident),*) => (concat!("", a!($($A),*)))
}

fn main() { 
    a! (A, A, A, A, A, A, A, A, A, A);
}
