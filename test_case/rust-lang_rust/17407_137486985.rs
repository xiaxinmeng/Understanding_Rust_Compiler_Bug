
macro_rules! go {
    () => { fn main() { undefined } };
    ($_x:tt $($rest: tt)*) => {
        go!($($rest)*);
    }
}

go!(1 2 3 4 5 6 7 8 9 10);
