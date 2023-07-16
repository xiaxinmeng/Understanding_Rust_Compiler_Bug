rust
fn with_mut<T, R> (
    p: &'_ mut UnsafeCell<T>,
    f: impl FnOnce(&'_ mut T) -> R,
) -> R
where
    T : Default,
{
    let mut prev = mem::take(p).into_inner();
    // defer putting the value back
    let mut prev = ::scopeguard::guard(prev, |it| *p = it.into());
    f(&mut *prev)
}
