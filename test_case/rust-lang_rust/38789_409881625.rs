rust
pub struct MyStruct {
    data: u32,
}

impl PartialEq for MyStruct {
    fn eq(&self, other: &Self) -> bool {
        return self.data == other.data;
    }
}
