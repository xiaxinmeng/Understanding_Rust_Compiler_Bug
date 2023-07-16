
trait MyIter<T> {
    fn myeach(&self, blk: &fn(&T) -> bool);
}

trait ExtendedIter<T> {
    fn myeachi(&self, blk: &fn(uint, &T) -> bool);
}

impl<T, IT: MyIter<T>> ExtendedIter<T> for IT {
    fn myeachi(&self, blk: &fn(uint, &T) -> bool) { }
}

impl<'a, T> MyIter<T> for &'a [T] {
    fn myeach(&self, blk: &fn(&T) -> bool) { }
}

fn main() {
    let v = ~[0];
    let iter = v.myeachi(|b, &a| {false});
}
