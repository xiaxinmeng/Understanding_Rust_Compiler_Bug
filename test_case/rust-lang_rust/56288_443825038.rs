rust
trait Alpha {
    fn build() -> &'static dyn Beta<Delta = ()>;
}

trait Beta: Gamma<Delta = Self> {}

trait Gamma {
    type Delta;
}
