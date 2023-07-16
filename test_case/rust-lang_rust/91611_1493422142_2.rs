
error[E0562]: `impl Trait` only allowed in function and inherent method return types, not in paths
  --> src\generation\iterator.rs:24:32
   |
24 |     async fn iterate(&self) -> impl Stream<Item = usize> + '_ {
