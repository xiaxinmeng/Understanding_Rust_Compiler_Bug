rust
#![crate_type="lib"]

type T = i32;

#[no_mangle]
pub fn id_result(a: Result<T, T>) -> Result<T, T> {
    match a {
        Ok(x) => Ok(x),
        Err(y) => Err(y),
    }
}
