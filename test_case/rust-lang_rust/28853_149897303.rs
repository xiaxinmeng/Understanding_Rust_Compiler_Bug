 rust
trait Borrow<T: ?Sized> {}
impl<T: ?Sized> Borrow<T> for T {}
impl<'a, T: ?Sized> Borrow<T> for &'a T {}

struct Map<K: 'static>(K);
impl<K> Map<K> {
    fn get<T>(&self, _key: &T) -> Option<&K> where K: Borrow<T> {
        loop {}
    }
}

struct Captures<'t>(&'t str);
impl<'t> Captures<'t> {
    fn at(&self) -> &'t str { loop {} }
}

pub fn foo(caps: &Captures,
           map: Map<&'static str>) {
    map.get(&caps.at());
}

fn main() {}
