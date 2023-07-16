 rust
#[feature(macro_rules)];

macro_rules! wrong {
    ($e: expr) => {
        (|x| { $e })(1)
    }
}

macro_rules! right {
    ($e: expr) => {
        { let x = 1; $e }
    }
}

fn main() {
    println!("{}", wrong!(x));
    // println!("{}", right!(x));
}
