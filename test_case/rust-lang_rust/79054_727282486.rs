rust
macro_rules! outer {
    ($x:literal) => {
        macro_rules! inner {
            ($x) => { true };
            ($_:literal) => { false };
        }
    };
}

outer!(5);

fn main() {
    assert!(inner!(5));
}
