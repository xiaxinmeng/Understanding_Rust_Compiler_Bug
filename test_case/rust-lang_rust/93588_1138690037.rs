rust
impl<T, ST> AsExpression<ST> for T
where
    T: Expression<SqlType = ST>,
    ST: SqlType + TypedExpressionType,
{
    type Expression = Self;

    fn as_expression(self) -> Self {
        self
    }
}
