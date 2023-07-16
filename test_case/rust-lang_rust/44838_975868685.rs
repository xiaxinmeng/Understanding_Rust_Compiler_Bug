rust
use anyhow::{ensure, Result};

#[derive(Debug, PartialEq)]
enum Kind {
    File,
    Directory,
    Symlink,
}

fn main() -> Result<()> {
    let kind = Kind::Symlink;
    ensure!(kind == Kind::File);
    Ok(())
}
