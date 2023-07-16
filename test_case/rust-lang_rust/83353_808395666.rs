rust
pub enum ThinBox<'a, T> {
    Borrowed(&'a T),
    Owned(Box<T>),
}

fn main() {
    dbg!(std::mem::size_of::<ThinBox<'static, &'static str>>()); //16
}
