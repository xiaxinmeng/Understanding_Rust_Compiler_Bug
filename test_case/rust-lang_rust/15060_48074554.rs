 rust
mod break1 {
    pub struct MyGuy;

    impl MyGuy {
        pub fn do1() { println!("do 1"); }
    }
}

impl break1::MyGuy {
    fn do2() { println!("do 2"); }
}

fn main() {
    break1::MyGuy::do1();
    break1::MyGuy::do2();
}
