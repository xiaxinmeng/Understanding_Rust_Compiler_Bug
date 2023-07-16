rust
pub trait Graph<K, V, E> {
    type Index: Debug + Display;
    type Res<T> = Result<T, GraphError<Self::Index>>;
    ...
}
