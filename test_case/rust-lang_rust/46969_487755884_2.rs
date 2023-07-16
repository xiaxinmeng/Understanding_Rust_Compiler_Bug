
trait O {
    type M;
}
trait U<A: O> {
    const N: A::M;
}
impl<D> O for D {
    type M = u8;
}
impl<C> U<C> for u16 where C: O<M=u32>{
    const N: C::M = 4;
}
