rust
macro_rules! test {
    ($lit:literal) => {
       test!(@ $lit);
    };
    (@ 1) => {
       println!("constant fragment");
    };
    (@ $lit:literal) => {
       println!("literal fragment");
    };
}

fn main() {
    test!(1);
    test!(@ 1);
}
