rust
fn mutate<T> (p: &mut UnsafeCell<T>, value: T)
{
    *p = value.into();
}
