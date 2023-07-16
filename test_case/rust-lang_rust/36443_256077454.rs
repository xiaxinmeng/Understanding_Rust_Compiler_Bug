 rust
pub fn new<S: AsRef<str>>(somedata: S) -> String {
    String::from(somedata.as_ref())
}
