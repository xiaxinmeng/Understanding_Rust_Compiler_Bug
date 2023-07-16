rust
#![windows_subsystem = "windows"]
use winsafe;

use std::fs;
use std::io::{self, Write};

fn main() -> io::Result<()> {
    let mut f = fs::File::create("output.txt")?;
    let commandline = winsafe::GetCommandLine();
    f.write_all(commandline.as_bytes())?;
    Ok(())
}
