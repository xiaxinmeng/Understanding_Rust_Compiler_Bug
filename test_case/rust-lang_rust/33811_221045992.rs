
impl<'a, A, R> FnOnce<A> for Box<FnBox<A, Output = R> + 'a> {
