rust
trait FooBad: for<'a> TheItem<'a>  {
    fn items(walker: &mut dyn for<'a> FnMut(<Self as TheItem<'a>>::Item));
}

trait TheItem<'a> {
    type Item;
}
