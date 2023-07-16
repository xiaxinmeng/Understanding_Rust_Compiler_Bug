rust
use std::convert::{TryFrom};

struct E;

impl From<!> for E {
    fn from(_: !) -> E {
        E
    }
}

struct F;

impl From<E> for F {
    fn from(_: E) -> F {
        F
    }
}

fn foo() -> Result<(), F> {
    u32::try_from(1u32)?;
    Ok(())
}
