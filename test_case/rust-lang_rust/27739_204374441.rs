 rust
trait Sum<Item> where Self: Sized + ::std::ops::Add<Item, Output=Self> {
    fn sum<I: Iterator<Item=Item>>(iter: I) -> Self;
}

pub trait Product<Item> where Self: Sized + ::std::ops::Mul<Item, Output=Self> {
    fn product<I: Iterator<Item=Item>>(iter: I) -> Self;
}
