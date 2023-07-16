rust
enum MyEnum {
    A,
    B,
}

macro_rules! my_macro {
    ($variant:ident, $x:expr) => {
        match $x {
            MyEnum::$variant => "there",
            _ => "world"
        }
    };
}

fn main() {
    let x = MyEnum::A;
    let y = my_macro!(A, x);
    println!("Hello {}", y);
}
