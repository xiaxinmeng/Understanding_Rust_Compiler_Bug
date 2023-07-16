rust
impl<T> DoStuff<T> for Foo where
    T: AsExpression<<T as Expression>::SqlType>,
    T::Expression: Expression,
    T: Expression
{}
