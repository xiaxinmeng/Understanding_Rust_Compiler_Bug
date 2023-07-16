rust
use std::ops::{ControlFlow, FromResidual, Try};

pub unsafe fn qux(x: Result<String, String>) -> Result<String, String> {
    match x.branch() {
        ControlFlow::Continue(res) => Result::from_output(res),
        ControlFlow::Break(err) => return Result::from_residual(err),
    }
}
