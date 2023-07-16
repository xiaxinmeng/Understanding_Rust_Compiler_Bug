 rust
#[derive(Copy)]
enum Error {
    EndOfFile = 0,
    WouldBlock = -1,
    // etc.
}

trait ToIsize {
    fn to_isize(&self) -> isize;
}

impl ToIsize for Result<usize, Error> {
    fn to_isize(&self) -> isize {
        match *self {
            Ok(u) => u as isize,
            Err(e) => e as isize,
        }
    }
}
