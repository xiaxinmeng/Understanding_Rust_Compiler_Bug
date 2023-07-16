 rust
use std::error::Error;
use std::fmt::{self, Display, Formatter};

#[derive(Debug)]
struct MyError { }

impl Error for MyError {
    fn description(&self) -> &str {
    "description"
}
}

impl Display for MyError {
    fn fmt(&self, formatter: &mut Formatter) -> Result<(), fmt::Error> {
        write!(formatter, "description")
    }
}

fn main() {
    let error: Box<Error> = From::from(Box::new(MyError {}));
    let downcasted = error.downcast_ref::<Box<MyError>>(); // Downcast to `Box<MyError>`
    println!("{:?}", downcasted); 

    let error: Box<Error> = Box::new(MyError {});
    let downcasted = error.downcast_ref::<MyError>();
    println!("{:?}", downcasted); // OK: prints Some(MyError).
}
