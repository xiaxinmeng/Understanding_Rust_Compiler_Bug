rust
// Unsuspecting user's code
#[allow(non_camel_case_types)]
struct i(i64);

macro_rules! ignorant_macro {
    () => {
        let i = 0;
        println!("{}", i);
    };
}

fn main() {
    // oh no!
    ignorant_macro!();
}
