rust
impl RangeArgument<usize> for usize {
    fn start(&self) -> Bound<usize> { Included(self) }
    fn end(&self) -> Bound<usize> { Included(self) }
}
