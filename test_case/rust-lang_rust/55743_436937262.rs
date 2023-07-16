 rust
use std::io::{BufRead, BufReader, Error};
use std::str;

fn main() -> Result<(), Error> {
    let mut f = BufReader::new(
        "Hello\nI'm a test\rThis is a second test\r\nand a third\n\rand the last one".as_bytes(),
    );

    loop {
        let length = {
            let buffer = f.fill_buf()?;
            let line_size = buffer
                .iter()
                .take_while(|c| **c != b'\n' && **c != b'\r')
                .count();
            if buffer.len() == 0 {
                break Ok(());
            }

            println!("{:?}", str::from_utf8(&buffer[..line_size]));

            line_size
                + if line_size < buffer.len() {
                    // we found a delimiter
                    if line_size + 1 < buffer.len() // we look if we found two delimiter
                    && buffer[line_size] == b'\r'
                    && buffer[line_size + 1] == b'\n'
                    {
                        2
                    } else {
                        1
                    }
                } else {
                    0
                }
        };

        f.consume(length);
    }
}
