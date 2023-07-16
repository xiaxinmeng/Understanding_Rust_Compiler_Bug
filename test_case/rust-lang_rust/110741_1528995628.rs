rs
pub struct Outer<'a, R>(Inner<'a, R>)
where
    R: for<'b> Fn(&'b bool, &'a u8);

pub struct Inner<'a, R>(&'a (), R);

unsafe impl<'a, R> Send for Inner<'a, R>
where
    R: for<'b> Fn(&'b bool, &'a u8)
{}

