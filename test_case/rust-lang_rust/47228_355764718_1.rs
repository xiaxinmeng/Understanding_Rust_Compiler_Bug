rust
struct Environment<'a, K, V> {
    prec: RangeMut<K, V>,
    succ: RangeMut<K, V>,
    _marker: PhantomData<&'a mut ()>,
}
fn environment(&mut self) -> Environment<K, V>
