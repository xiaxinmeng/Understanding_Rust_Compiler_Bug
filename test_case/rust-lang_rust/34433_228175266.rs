
impl<A, B> ExactSizeIterator for Chain<A, B> where
    A: ExactSizeIterator,
    B: ExactSizeIterator<Item=A::Item>, {}
