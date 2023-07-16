rust
use std::ops::ControlFlow;
pub fn demo(x: Result<u32, i32>) -> ControlFlow<i32, u32> {
    match x {
        Ok(v) => ControlFlow::Continue(v),
        Err(e) => ControlFlow::Break(e),
    }
}
