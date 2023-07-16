 rust
fn main() {
    struct ByRef<'r, I: ?Sized>(&'r mut I) where I: 'r;
    impl<'r, I: ?Sized> Iterator for ByRef<'r, I> where
        I: 'r + Iterator
    {
        type Item = <I as Iterator>::Item;
        fn next(&mut self) -> Option<<I as Iterator>::Item>
        {
            self.0.next()
        }
    }

    let mut it = Box::new(0..10) as Box<Iterator<Item=i32>>;
    assert_eq!(it.next(), Some(0));

    let mut jt: &mut Iterator<Item=i32> = &mut *it;
    assert_eq!(jt.next(), Some(1));

    let mut r = ByRef(jt);
    assert_eq!(r.next(), Some(2));    // No ICE without this line.
}
