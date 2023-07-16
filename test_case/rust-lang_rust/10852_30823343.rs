
trait MutableSeq<T> {
    fn push(&mut self, t: T);
    fn pop(&mut self) -> Option<T>;
    fn insert(&mut self, idx: uint, t: T);
    fn remove(&mut self, idx: uint) -> Option<T>;
    ...
}

trait MutableDeque<T>: MutableSeq<T> {
    fn push_front(&mut self, t: T);
    fn pop_front(&mut self) -> Option<T>;
    ...
}
