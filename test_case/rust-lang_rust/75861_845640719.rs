
fn new_cyclic_2<A, B>(data_fn: impl FnOnce(&Weak<A>, &Weak<B>) -> (A, B)) -> (Arc<A>, Arc<B>) {
 ...
}
