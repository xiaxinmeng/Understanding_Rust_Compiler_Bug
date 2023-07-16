rust
enum Error {
    Custom(String),
}

impl Error {
    //  or T: ToString
    fn new<T: std::fmt::Display>(t: T) -> Self {
        Self::Custom(t.to_string())
    }
}

fn main() {
    let stuff = 1234;
    let _a = Error::new(stuff);
    let _b = Error::new(format_args!("Format args: {stuff}"));
    let _c = Error::new(format!("Format: {stuff}"));
    let _d = Error::new("Str");
}
