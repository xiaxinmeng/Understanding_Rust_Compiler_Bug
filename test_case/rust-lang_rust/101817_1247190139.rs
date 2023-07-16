rust
use std::error::Error;

#[derive(Debug, PartialEq, Eq)]
struct E;

static E2: E = E;

impl std::fmt::Display for E {
    fn fmt<'a>(&self, f: &mut std::fmt::Formatter<'a>) -> std::fmt::Result {
        write!(f, "E")
    }
}

impl Error for E {
    fn description(&self) -> &'static str {
        "E"
    }

    fn cause(&self) -> Option<&Error> {
        Some(&E2)
    }
}

#[test]
fn custom_io_has_source() {
    let e = E;
    let e = std::io::Error::new(std::io::ErrorKind::Other, e);
    assert!(e.cause().is_some());
}
