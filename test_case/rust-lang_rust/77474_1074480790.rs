rust
use thiserror::Error;

fn main() {
    let err = inner().unwrap_err();
    println!("Error: {:?}", err);
}

#[derive(Debug, Error)]
#[error("my error")]
struct MyError;

fn inner() -> eyre::Result<()> {
    Err(MyError)?
}
