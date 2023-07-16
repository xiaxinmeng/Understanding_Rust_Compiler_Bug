rust
> const SIZE_OF<T>: usize = std::mem::size_of::<T>();
> 
> trait IsEqual: 'static {
>     const TO<U: 'static>: bool;
> }
> impl<T: 'static> IsEqual for T {
>     const TO<U: 'static>: bool = TypeId::of::<T>() == TypeId::of::<U>();
> }
> 