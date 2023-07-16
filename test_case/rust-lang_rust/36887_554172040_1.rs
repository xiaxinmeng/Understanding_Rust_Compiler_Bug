rust
enum FilterOp<T=()> {
    Binary(T),
    Unary
}
trait Filter<T=()> {
    fn filter(self, field: &str, op: FilterOp<T>) -> Self;
    ...
}

impl<T=()> Filter<T> for Query {
    fn filter(self, field: &str, op: FilterOp<T>) -> Self {
        ...
    }
}
