rust
use std::io::*;

fn main() {
    let in_buf: &[u8] = b"a\nb\nc";
    let mut reader = BufReader::with_capacity(2, in_buf);
    let elem = first_line(&mut reader);
    assert_eq!(elem, "a");
}

fn first_line<R: Read>(reader: &mut BufReader<R>) -> String {
    let elem = reader.lines().next().unwrap().unwrap();
    elem
}
