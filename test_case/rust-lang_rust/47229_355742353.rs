
use std::fs::File;
use std::io::Read;
use std::io;
fn read(mut file: &File) -> io::Result<()> {
    let mut buf = [0; 4];
    file.read(&mut buf)?;
    Ok(())
}
