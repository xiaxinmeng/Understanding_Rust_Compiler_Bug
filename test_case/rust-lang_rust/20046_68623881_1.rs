
fn main() {
    match std::io::stdio::stdin().read_byte() {
        Ok(_) => (),
        Err(ref e) if e.kind == std::io::EndOfFile => (),
        Err(std::io::IoError { kind: EndOfFile, .. }) => (),
    }
}
