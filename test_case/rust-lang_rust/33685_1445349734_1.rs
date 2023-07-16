
pub trait FnOnce<Args: Tuple, Output> {
    fn call_once(self, args: Args) -> Output;
}
