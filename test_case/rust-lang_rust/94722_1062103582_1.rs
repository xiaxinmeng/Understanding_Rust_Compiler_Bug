rust
trait Trait<T> {
    fn do_stuff(&self) {}
}

impl<T> Trait<T> for T {}

fn main() {
    [1,2,3].do_stuff(); // this method works, so all [U; N] types are already covered
    ().do_stuff();
}
