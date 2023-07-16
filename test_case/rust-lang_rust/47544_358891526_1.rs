rust
fn main() -> Result<(), Box<Error>> { 
    Err(Box::new(Error::new(ErrorKind::Other, "returned Box<Error> from main()")))
}
