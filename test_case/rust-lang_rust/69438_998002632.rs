rust
trait MyClone {
    fn clone_box(&self) -> Box<dyn MyClone>;
}
impl<T: Clone + 'static> MyClone for T {
    fn clone_box(&self) -> Box<dyn MyClone> {
        Box::new(self.clone())
    }
}

fn main() {
    let x = Box::new(123456);
    let x: Box<dyn MyClone> = x;
    let _y = x.clone_box();
}
