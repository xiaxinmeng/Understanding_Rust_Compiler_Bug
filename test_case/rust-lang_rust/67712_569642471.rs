rust
match some_slice: &[T] {
    &[] => "zero",
    &[_a] => "one",
    &[_a, _b] => "two",
    &[_a, _b, _c, ..] => "many",
}
