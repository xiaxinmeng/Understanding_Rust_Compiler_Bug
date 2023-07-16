rust
impl<T, ST> AsExpression<ST> for T
where
    T: Expression<SqlType = ST>,
    ST: SqlType + TypedExpressionType,
{
    type Expression = T;

    fn as_expression(self) -> T {
        self
    }
}
