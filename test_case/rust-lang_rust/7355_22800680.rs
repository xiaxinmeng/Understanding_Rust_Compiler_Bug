
trait map<T> {
    fn map<U>(&self, f: &fn(&T) -> U) -> ~[U];
}
impl<T> map<T> for ~[T] {
    fn map<U>(&self, f: &fn(&T) -> U) -> ~[U] {
        let mut r: ~[U] = ~[];
        // for x in self.iter() { r.push(f(x)); } // works fine
        for x in self.iter() { r += ~[f(x)]; }
        r
    }
}

pub fn main() { }
