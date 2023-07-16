
trait DerefMove<E> : DerefImm<E> {
    fn deref_move(self) -> E;
}
