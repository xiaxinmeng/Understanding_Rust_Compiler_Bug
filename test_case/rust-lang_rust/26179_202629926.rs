
enum Result<T, E> {
   #[likely]
   Ok(T),
   Err(E)
}
