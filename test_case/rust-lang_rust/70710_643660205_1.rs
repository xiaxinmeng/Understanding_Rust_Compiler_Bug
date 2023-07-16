rust
fn fmt(&self, f: &mut fmt:Formatter) -> fmt:Result {
    let f = if f.precision.is_some() {
        f
    } else {
        f.set_precision(1)
    }
    fmt::Debug::fmt(self, f)
}
