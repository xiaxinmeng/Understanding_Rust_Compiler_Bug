 rust
fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    if f.alternate_flag_enabled() {
        write!(f, "({:#?},)", self.0)
    } else {
        write!(f, "({:?},)", self.0)
    }
}
