rust
struct ClosureType;
impl<'a> Fn<()> for ClosureType {
    type Output = &'a str;
}
