rust
macro_rules! s {
    ($e: expr) => {
        String::from($e)
    }
}

pub fn foo(x: &str) -> Result<(), Box<dyn std::error::Error>> {
    Err(s!(x))
}
