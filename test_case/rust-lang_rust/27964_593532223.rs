rust
use std::error::Error;

fn library_function<F1, E1>(f1: F1) -> Result<(), E1>
where
    F1: FnOnce() -> Result<(), E1>,
    E1: Error, // <-remove this line -> compile
{
    f1()
}

fn main() -> Result<(), Box<dyn Error>> 
{
    library_function( || Ok(()) )
}
