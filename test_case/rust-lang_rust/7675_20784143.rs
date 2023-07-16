
#[allow(default_methods)];

pub trait Container {
    fn is_empty(&self) -> bool { true }
}

pub struct HashMap<K,V>;

impl<K:Hash + Eq,V> Container for HashMap<K, V> {
}

pub struct HashSet<T> {
    priv map: HashMap<T, ()>
}


impl<T:Hash + Eq> Container for HashSet<T> {

    /// Return true if the set contains no elements
    fn is_empty(&self) -> bool { self.map.is_empty() }
}

fn argh(x: &HashSet<int>) -> bool {
    x.is_empty()
}

fn main () {}
