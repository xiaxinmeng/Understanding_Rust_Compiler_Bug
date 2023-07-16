rust
macro_rules! go {
    () => { fn main() { undefined } };
    (a $_x:tt $($rest: tt)*) => {
        go!(b $($rest)*);
    };
    (b $_x:tt $($rest: tt)*) => {
        go!(a $($rest)*);
    };
    ($_x:tt) => {
        go!();
    };
}

go!(a 1 2 3 4 5 6 7 8 9 10);
