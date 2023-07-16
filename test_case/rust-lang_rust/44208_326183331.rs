
trait DataTable<V> {
    fn find(&self, needle: char) -> Option<V>;
}

impl<'a, V> DataTable<&'a V> for &'a [(char, V)] {
    fn find(&self, needle: char) -> Option<&'a V> {
        self.binary_search_by_key(&needle, |&(k, _)| k)
            .map(|idx| &self[idx].1)
            .ok()
    }
}

impl DataTable<()> for &'static [(char, char)] {
    fn find(&self, _: char) -> Option<()> {
        Some(())
    }
}

fn main() {
    static TABLE: &[(char, char)] = &[('a', 'a')];
    let x: Option<char> = TABLE.find('a').cloned();
}
