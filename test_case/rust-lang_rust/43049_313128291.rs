rust
impl<T> DoStuff<T> for Foo where
    T: AsExpression<<Self as Expression>::SqlType>,
    T::Expression: Expression,
    T: Expression
{}
