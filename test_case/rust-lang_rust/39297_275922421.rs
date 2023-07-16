rust
trait Deserialize: Sized {
    fn deserialize() -> Result<Self, String>;
}

impl Deserialize for () {
    fn deserialize() -> Result<(), String> {
        Ok(())
    }
}

impl Deserialize for ! {
    fn deserialize() -> Result<!, String> {
        Err("Failed to deserialize a `!`")
    }
}

fn doit() -> Result<(), String> {
    let _ = <_ as Deserialize>::deserialize()?;
    Ok(())
}

fn main() {
    let _ = doit();
}
