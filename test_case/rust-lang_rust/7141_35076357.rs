
trait DerefImm<E> {
    fn deref<'a>(&'a self) -> &'a E;
}

trait DerefMut<E> : DerefImm<E> {
    fn deref<'a>(&'a mut self) -> &'a mut E;
}
