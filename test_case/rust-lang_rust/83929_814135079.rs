
type Result<T, E = impl std::error::Error> = std::result::Result<T, E>;

fn main() -> Result<()> {
    Ok(())
}
