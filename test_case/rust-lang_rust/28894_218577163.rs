 Rust
pub struct Text;

pub trait Expression {
    type SqlType;
}

pub trait AsExpression<ST> {
    type Expression: Expression<SqlType=ST>;

    fn as_expression(self) -> Self::Expression;
}

impl<T, ST> AsExpression<ST> for T where
    T: Expression<SqlType=ST>,
{
    type Expression = Self;

    fn as_expression(self) -> Self::Expression {
        self
    }
}

fn main() {
    AsExpression::as_expression(Text);
}
