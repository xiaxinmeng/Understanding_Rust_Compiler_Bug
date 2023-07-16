rust
> impl<T=()> Filter<T> for Query {
>     fn filter(self, field: &str, op: FilterOp<T>) -> Self {
>         ...
>     }
> }
> 