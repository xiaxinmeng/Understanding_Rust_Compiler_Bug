rust
enum MyNonSyncOption<T> {
  NonSync(Cell<()>),
  Some(T),
  None
}
