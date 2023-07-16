 rust
#![crate_type="lib"]
trait Chromosome<C: Chromosome<C>> {
    fn random() -> C;
}
