
trait MaybeOwned<T> {
    fn maybe_get_mut<'s>(&'s mut self) -> Option<&'s mut T>;
}

trait CopyOnWrite<T>: MaybeOwned<T> {
    fn cow_get_mut<'s>(&'s mut self) -> &'s mut T;
}
