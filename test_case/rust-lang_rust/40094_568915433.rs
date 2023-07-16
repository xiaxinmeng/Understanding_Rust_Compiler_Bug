rust
#[derive(Debug)]
enum Error {
    Io(std::io::Error),
    Xml(std::io::Error),
    Other,
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "an error")
    }
}


impl std::error::Error for Error {
    fn cause(&self) -> Option<&dyn std::error::Error> {
        match *self {
            Error::Io(ref e) => Some(e),
            Error::Xml(e) => Some(&e), // <- problematic arm
            _ => None
        }
    }
}
