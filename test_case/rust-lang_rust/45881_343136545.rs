rust
fn leak<'a>(b: Box<T>) -> &'a mut T
where
    T: 'a,
{
    unsafe { &mut *Box::into_raw(b) }
}
