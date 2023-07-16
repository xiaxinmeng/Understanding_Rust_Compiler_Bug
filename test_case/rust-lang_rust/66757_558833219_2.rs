rust
use std::convert::TryFrom;

struct F;

fn foo() -> Result<(), F> {
    u32::try_from(1u32)?;
    Ok(())
}
