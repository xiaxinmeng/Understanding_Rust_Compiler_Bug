rust
trait DataTable<V> {
    fn find(&self, needle: char) -> Option<V>;
}

impl<'a, V> DataTable<&'a V> for &'a [(char, V)] {
    fn find(&self, _: char) -> Option<&'a V> {
        None
    }
}

impl DataTable<()> for &'static [(char, char)] {
    fn find(&self, _: char) -> Option<()> {
        None
    }
}

fn main() {
    static TABLE: &[(char, char)] = &[('a', 'a')];
    let x: Option<char> = TABLE.find('a').cloned();
}
