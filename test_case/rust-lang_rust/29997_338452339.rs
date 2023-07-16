rust
pub trait StreamingIterator<'a> {
    type Item: 'a;

    fn next(&'a mut self) -> Option<Self::Item>;
}

pub trait StreamingIteratorExt: for<'a> StreamingIterator<'a> {
    fn for_each<F>(mut self, mut func: F) where
        Self: Sized,
        F: for<'a> FnMut(<Self as StreamingIterator<'a>>::Item)
    {
        while let Some(item) = self.next() {
            func(item)
        }
    }
}

impl<I> StreamingIteratorExt for I where
    I: for<'a>StreamingIterator<'a>
{}

impl<'a, I: Iterator + 'a> StreamingIterator<'a> for I {
    type Item = I::Item;

    fn next(&'a mut self) -> Option<Self::Item> {
        self.next()
    }
}

#[test]
fn test_basic_stuff() {
    let it = vec![1i32,2,3].into_iter();

    StreamingIteratorExt::for_each(it, |x| println!("{}", x));
}

