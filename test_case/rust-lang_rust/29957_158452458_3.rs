
fn main() {
    let result = try_read();

    if result.is_err() {
        println!("there was an error!");
    }
}

fn try_read() -> std::io::Result<()> {
    use std::io::prelude::*;
    use std::io::BufReader;
    use std::fs::File;

    let mut f = try!(File::open("log.txt"));
    let mut reader = BufReader::new(f);

    let mut line = String::new();
    let len = try!(reader.read_line(&mut line));
    println!("First line is {} bytes long", len);

    Ok(())
}
