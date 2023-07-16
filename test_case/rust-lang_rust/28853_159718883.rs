 rust
trait Borrow<T: ?Sized> {}
impl<T: ?Sized> Borrow<T> for T {}

struct Map<K>(K);
impl<K> Map<K> {
    fn get<T>(&self, _key: &T) -> Option<&u32> where K: Borrow<T> {
        loop {}
    }
}

struct Captures<'t>(&'t str);
impl<'t> Captures<'t> {
    fn at(&self) -> &'t str { loop {} }
}

fn foo<'a>(names: &[&str],
           map: &'a Map<&'static str>)
           -> Option<&'a u32> {
    map.get(&names[0])
}

fn main() {}
