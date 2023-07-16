
trait Coercible<T> { }
#[inline(always)]
fn coerce<U, T: Coercible<U>>(x: T) -> U { unsafe { transmute(x) } }
