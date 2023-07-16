rust
type T = (i32, i32);
type E = T;
type R = Result<T, E>;

#[no_mangle]
pub fn try_op(a: R) -> R {
    Ok(a?)
}

#[no_mangle]
pub fn try_macro(a: R) -> R {
    Ok(try!(a))
}
