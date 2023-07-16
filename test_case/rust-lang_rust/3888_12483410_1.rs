
fn vec_peek<T>(v: &r/[T]) -> &r/[T] {
    vec::view(v, 1, 5)
}
