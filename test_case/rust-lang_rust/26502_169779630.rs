 rust
trait Trait {
    fn default(&self, name: String) {
        drop(name);
    }
}
