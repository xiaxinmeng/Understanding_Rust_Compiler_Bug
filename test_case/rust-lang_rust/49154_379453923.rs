rust
#[derive(Debug)]
pub enum Error {
    Type(
        &'static str,
    ),
}
