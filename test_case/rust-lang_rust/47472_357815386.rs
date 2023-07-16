rust
mod my_module {
    #[derive(Default)]
    pub struct MyStruct {
        pub public_field: usize,
        private_field: bool,
        another_private_field: bool,
    }
}
 
use my_module::MyStruct;
 
fn main() {
    let s = MyStruct { public_field: 2, ..Default::default() };
}
