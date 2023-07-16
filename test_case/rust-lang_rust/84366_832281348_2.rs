rust
struct ClosureType<R>;
impl<'a> Fn<()> for ClosureType<&'a str> {
    type Output = &'a str;
}
