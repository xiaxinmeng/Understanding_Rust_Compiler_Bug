rust
fn unconstrained_err<T>() -> Result<u32, T> { Ok(22) }

fn main() {
    let _ = if true {
      unconstrained_err()
    } else {
      Err(panic!())
    };
}
