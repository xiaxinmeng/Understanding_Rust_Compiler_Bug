 rust
impl String {
    fn concat<S, It>(iter: It) -> String
        where S: Str, It: Iterator<S> { ... }
}
