rs
pub struct Payload {
    borrow: isize,
    value: [u8; 1000],
}

pub struct MyStruct<T: ?Sized> {
    pub flags: u8,
    pub next: Option<i32>,
    pub value: T,
}
