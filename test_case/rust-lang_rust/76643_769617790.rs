rust
//Oops, I forgot a #[derive(Clone)]
struct MyStruct;

impl MyStruct {
    fn my_clone(&self) -> MyStruct {
        self.clone()
    }
}
