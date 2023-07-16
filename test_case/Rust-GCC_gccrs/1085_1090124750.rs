rust
macro_rules! add {
    ($e:expr, $($es:expr),*) => {
        $e + add!($($es),*)
    };
    ($e:expr) => {
        $e
    };
}

fn main() -> i32 {
    let b = add!(add!(15, 20), add!(3,4));

    0
}
