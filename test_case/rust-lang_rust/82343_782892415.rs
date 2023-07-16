rust
fn main() -> io::Result<()> {
    if bad {
        // creates a single use type that implements `IoError` with these values
        // As a ZST, no allocations have to be made to return it.
        return io::err!(Other; "*crickets*");
        // Naturally, there are many variations to this - any `repr(int)` enum could be packed
        // into the data half of the pointer. One could declare a `NotFound` type that
        // held a `&'static ThinStr` and always returned `ErrorKind::NotFound`, etc...
    }
    Ok(())
}
