 rust
fn every_other_element<T: Copy>(
    values: &[T]
) -> ~[T] {
    match values {
        [head, _, tail...] => ~[head] + every_other_element(tail),
        [head] => ~[head],
        [] => ~[]
    }
}
