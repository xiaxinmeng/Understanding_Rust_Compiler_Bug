 rust
macro_rules! foo {
    ($ex:expr) => {
        match $ex {
            Some(ouch) => println!("{}", ouch),
            None => println!("none")
        }
    }
}

pub fn main() {
    const ouch: i32 = 123;
    foo!(Some(567));
}
