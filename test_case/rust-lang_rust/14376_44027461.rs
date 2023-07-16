 rust
pub trait Map<'a, K, V, EI: Iterator<(&'a K, &'a V)>, KI: Iterator<&'a K>, VI: Iterator<&'a V>> {
    // ...
    fn keys(&'a self) -> KI;
    fn values(&'a self) -> VI;
    fn entries(&'a self) -> EI;
}
