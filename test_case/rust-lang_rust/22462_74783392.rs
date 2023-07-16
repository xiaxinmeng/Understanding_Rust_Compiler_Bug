 rust
macro_rules! foo {
    ($e:expr) => {
        match $e {
            x => println!("{}", x),
        }
    }
}

fn main() {
    const x: u32 = 2;
    foo!(3);
}
