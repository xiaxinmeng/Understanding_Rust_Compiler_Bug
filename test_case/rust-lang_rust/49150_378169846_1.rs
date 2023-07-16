rust
#[async]
fn foo<'a, R>(source: Pin<'a, R>) -> Result<!, Error> where R: Read + 'a {
    loop {
        let mut buffer = [0; 8];
        await!(read_exact(Pin::borrow(&mut source), &mut buffer[..]));
        // do something with buffer
    }
}
