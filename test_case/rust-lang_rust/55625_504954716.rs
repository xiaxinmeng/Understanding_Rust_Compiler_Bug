rust
enum B<T> {
    B1(T),
    B2(Box<B<B<T>>>),
    B3(Box<B<Option<B<T>>>>),
}
