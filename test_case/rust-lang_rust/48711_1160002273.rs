rust
use std::process::{ExitCode, Termination};

#[derive(Debug)]
pub enum MyError {
    Internal,
    Other,
}

impl Termination for MyError {
    fn report(self) -> ExitCode {
        match self {
            MyError::Internal => ExitCode::from(1),
            MyError::Other => ExitCode::from(255),
        }
    }
}

fn main() -> Result<(), MyError> {
    Err(MyError::Other)?;
    Ok(())
}
