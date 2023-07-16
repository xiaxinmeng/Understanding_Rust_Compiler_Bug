 rust
match r.read(buf) {
    Ok(bytes) => { /* process buf */ }
    Err(IoError { kind: EndOfFile, .. }) => { /* bail out */ }
    Err(err) => { /* handle error */ }
}
