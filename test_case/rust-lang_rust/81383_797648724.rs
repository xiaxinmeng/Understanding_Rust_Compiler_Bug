rust
use std::convert::Infallible;

pub enum MyResult<T, E> {
    Ok(T),
    Err(E),
}

trait UnsafeUnwrapUnchecked<T> {
    unsafe fn unwrap_unchecked(self) -> T;
}

impl<T, E> UnsafeUnwrapUnchecked<T> for MyResult<T, E> {
    unsafe fn unwrap_unchecked(self) -> T {
        match self {
            MyResult::Ok(t) => t,
            MyResult::Err(_) => core::hint::unreachable_unchecked(),
        }
    }
}

impl<T> MyResult<T, Infallible> {
    fn unwrap_unchecked(self) -> T {
        match self {
            MyResult::Ok(t) => t,
            MyResult::Err(_) => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}
    
pub fn f() {
    let _ = MyResult::<i32, Infallible>::Ok(42).unwrap_unchecked();
    let _ = unsafe { MyResult::<i32, i32>::Ok(42).unwrap_unchecked() };
}
