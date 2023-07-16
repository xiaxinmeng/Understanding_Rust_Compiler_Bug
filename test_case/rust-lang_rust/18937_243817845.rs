 Rust
pub trait Leak<T : ?Sized> {
    fn leak<'a>(self) -> &'a T where T: 'a;
}

impl<T> Leak<[T]> for Vec<T> {
    fn leak<'a>(mut self) -> &'a [T] where [T]: 'a {
        let r: *mut [T] = &mut self[..];
        std::mem::forget(self);
        unsafe { &mut *r }
    }
}
fn main() {
    println!("Hello, world!");
}
