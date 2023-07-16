rust
type TransactionFuture<'__, O> = impl '__;
fn execute_transaction_fut<'f, F, O>() -> FnOnce(&Transaction) -> TransactionFuture<'_, O> {
    f
}
