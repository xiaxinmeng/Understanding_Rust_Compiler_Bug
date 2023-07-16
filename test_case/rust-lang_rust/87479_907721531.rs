
pub trait MutPointer<T>:
{
    type Cast<U>: MutPointer<U>;

    fn cast<U>(self) -> Self::Cast<U>;

    // ... other less relevant methods
}
