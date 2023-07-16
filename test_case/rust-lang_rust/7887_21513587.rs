
trait ConsumableIterator<T> {
    fn map<U>(&self, f: &fn(T) -> U) -> ConsumableIterator<U>;
}

trait FromIterator<T> {
    static fn from_consume_iter(i: ConsumableIterator<T>) -> Self;
}

trait Mappable<T>: ConsumableIterator<T> {
    fn map<U, Container: FromIterator<U>>(self, f: &fn(T) -> U) -> Container<U> {
        FromIterator::from_consume_iter(self.consume_iter().map(f))
}

impl<T> Mappable<T> for ~[T] {}
