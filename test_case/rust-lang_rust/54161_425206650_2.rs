rust
struct MyStruct;

mod some_mod {
    pub trait MyTrait {
        fn hello(&self) {}
    }
    impl MyTrait for ::MyStruct {}
}

fn main() {
    let a = MyStruct {};
    a.helloo();
}
