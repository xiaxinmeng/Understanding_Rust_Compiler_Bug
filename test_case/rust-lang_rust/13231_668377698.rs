rust
struct S;
impl std::error::Error for S {}
impl std::fmt::Display for S {fn fmt(&self, _f: &mut std::fmt::Formatter) -> std::fmt::Result {Ok(())}}
impl !Something for S {}
foo(Error::new(ErrorKind::Other, S));  // io::Error containing an S.
