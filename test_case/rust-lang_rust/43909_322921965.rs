rust
impl Debug for () {
    fn fmt(&self, f: &mut Formatter) -> Result {
        f.pad("()")
    }
}
