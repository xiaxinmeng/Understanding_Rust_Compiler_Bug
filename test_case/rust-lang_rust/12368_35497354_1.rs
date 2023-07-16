 rust
let (mut reader, mut handle) = ChompingReader::new(my_reader);

// no failure, but errors are recorded, and the loop quits immediately...
for line in reader.lines() {
    foo(line);
}
// ... so handling the error here immediately after the loop 
// is pretty similar to handling it manually with .read_line.
match handle.error() {
    Err(e) => println!("error occured in `.lines`)
    Ok(()) => {}
}
