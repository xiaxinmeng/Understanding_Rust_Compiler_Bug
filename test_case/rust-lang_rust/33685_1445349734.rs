
pub trait FnOnce<Args: Tuple> {
    type Output;
    fn call_once(self, args: Args) -> Self::Output;
}
