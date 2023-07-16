 rust
use std::fmt::{Display, Formatter, Error};

struct MyLocalType;

type MyResult = Result<MyLocalType, String>;

impl Display for MyResult {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        f.write_str("some test string")
    }
}

fn main() { 
    let r: MyResult = Ok(MyLocalType); 
    println!("{}" , r); 
}
