
trait MySortByKey<T> {
    fn my_sort_by_key<'a, B, F>(&mut self, f: F)
    where
        B: 'a + Ord,
        T: 'a,
        F: FnMut(&'a T) -> B;
}

impl<T> MySortByKey<T> for [T] {
    fn my_sort_by_key<'a, B, F>(&mut self, f: F)
    where
        B: 'a + Ord,
        T: 'a,
        F: FnMut(&'a T) -> B,
    {
        let a = f(&self[0]);
        let b = f(&self[1]);
        
        match a.cmp(&b) {
            _ => self.swap(0, 1),
        }
    }
}

fn main() {}
