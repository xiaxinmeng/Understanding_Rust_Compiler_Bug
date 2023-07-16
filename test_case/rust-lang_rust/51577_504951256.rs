rust
pub type Result<T> = std::result::Result<T, ()>;

pub mod example1 {
    use std::result::Result as StdResult;

    pub type Result<T> = StdResult<T, ()>;
}

pub mod example2 {
    pub type Result<T> = std::result::Result<T, ()>;
}
