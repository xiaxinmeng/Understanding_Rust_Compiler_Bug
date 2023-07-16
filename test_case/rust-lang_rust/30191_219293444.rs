
trait Convert {
    fn convert(&self) -> i64;
}

impl<T> Convert for T
    where T: Copy,
          i64: From<T>
{
    fn convert(&self) -> i64 {
        i64::from(*self)
    }
}

impl Convert for f64 {
    fn convert(&self) -> i64 {
        unsafe { std::mem::transmute(*self) }
    }
}
