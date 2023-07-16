rust
fn fmt(&self, f: &mut fmt:Formatter) -> fmt:Result {
    if f.precision.is_some() {
        fmt::Debug::fmt(self, f)
    } else {
        write!(f, "{:.1?}", self)
    }
}
