
struct E;

trait Fallible {
    type Error: Into<E>;
    fn e(&self) -> Result<(), Self::Error>;
}

fn test(f: &impl Fallible) -> Result<(), E> {
    Ok(f.e()?)
}
