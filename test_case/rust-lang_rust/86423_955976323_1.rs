rust
use std::io::prelude::*;
use std::io::BufReader;
use std::fs::File;

fn main() -> std::io::Result<()> {
    let f = File::open("log.txt")?;
    let mut reader = BufReader::new(f);

    while let Some(amazing_log_line) = reader.fill_buf2()? {
        println!("{:?}", amazing_log_line);
        read.consume(amazing_log_line.len());
    }

    Ok(())
}
