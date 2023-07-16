 rust
mod hidden {
    pub trait TestTrait {
        fn do_something(&self);
    }

    pub struct TestStruct;

    impl TestTrait for TestStruct {
        fn do_something(&self) { println!("Hello world"); }
    }
}

fn main() {
    let x = hidden::TestStruct;
    x.do_something();
}
