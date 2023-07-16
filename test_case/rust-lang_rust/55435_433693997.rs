rust
macro_rules! my_crate {
    ($crate:tt) => {
        println!("Hello!");
    }
}

macro_rules! check {
    () => {
        my_crate!($crate:tt);
    }
}

fn main() {
    check!();
}
