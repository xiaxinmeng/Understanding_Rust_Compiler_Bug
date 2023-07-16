rust
impl fmt::Display for NewType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "NewType: {}", **self)
        //                       ^^
    }
}
