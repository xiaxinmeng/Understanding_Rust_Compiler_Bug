Rust
struct StringError(Box<str>);

struct BoxErrorMapper;
impl ErrorOrStrMapper for BoxErrorMapper {
    type Output = Box<Error + Send + Sync>;
    fn from_error<E: Error>(self, e: E) -> Self::Output {
        Box::new(e)
    }
    fn from_str(self, s: &str) -> Self::Output {
        // I'm not sure on that `From` - we might want to return an
        // OomError here if we fail to allocate enough memory.
        Box::new(StringError(From::from(s)))
    }
}

impl<E: ErrorOrStr> From<E> for Box<Error + Send + Sync> {
    fn from(err: E) -> Self {
        err.to_error(BoxErrorMapper)
    }
}

impl<E: ErrorOrStr> From<E> for Box<Error> {
    fn from(err: E) -> Self {
        err.to_error(BoxErrorMapper)
    }
}

impl From<Box<str>> for Box<Error + Send + Sync> {
    fn from(s: Box<str>) -> Self {
        Box::new(StringError(s))
    }
}

// ok because String is local, so we know `!String: ErrorOrStr`
impl From<Box<str>> for Box<Error> {
    fn from(s: String) -> Self {
        Box::new(StringError(s))
    }
}
