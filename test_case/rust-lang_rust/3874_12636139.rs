
macro_rules! a_macro {
    ($zero:expr) => {
        pub fn foo() -> int {
            let x:int = $zero;
            x + $zero.add(&1)
        }
    }
}

a_macro!(0i)

fn main() { }
