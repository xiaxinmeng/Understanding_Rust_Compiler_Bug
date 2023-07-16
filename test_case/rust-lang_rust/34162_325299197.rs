
trait MySortByKey<T> {
    fn my_sort_by_key<B, F>(&mut self, f: F)
    where
        B: Ord,
        F: for<'a> BorrowFn<'a, T, B>;
}

trait BorrowFn<'a, T: 'a, B: 'a>: FnMut(&'a T) -> B {}
impl<'a, T: 'a, B: 'a, F: FnMut(&'a T) -> B> BorrowFn<'a, T, B> for F {}

impl<T> MySortByKey<T> for [T] {
    fn my_sort_by_key<B, F>(&mut self, mut f: F)
    where
        B: Ord,
        F: for<'a> BorrowFn<'a, T, B>,
    {
        let a = f(&self[0]);
        let b = f(&self[1]);
        
        match a.cmp(&b) {
            _ => self.swap(0, 1),
        }
    }
}

fn main() {}
