
pub trait MutPointer<P<T>>:
{
    fn cast<U>(self) -> P<U>;
}
