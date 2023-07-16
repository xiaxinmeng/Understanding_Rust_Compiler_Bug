rust
trait FooBad: for<'a> TheItem<'a> {
    fn items<F>(walker: F)
    where
        F: FnOnce(<Self as TheItem<'_>>::Item);
}

trait TheItem<'a> {
    type Item;
}

struct Struct<T>(T);

impl<'a, T> TheItem<'a> for Struct<T>
where 
    Self: 'a,
{
    type Item = &'a mut T;
}

impl<T> FooBad for Struct<T> {
}
