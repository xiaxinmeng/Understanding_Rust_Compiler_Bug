 rust
trait Hash<H: Hasher<Self>> {
    fn f(&self, _: H) {}
}

trait Hasher<H: Hash<Self>> {
    fn g(&self, _: H) {}
}

trait StreamHash<H: StreamHasher<Self>>: Hash<H> {
    fn h(&self, _: H) {}
}

trait StreamHasher<H: StreamHash<Self>>: Hasher<H> {
    fn i(&self, _: H) {}
}

fn main() {
}
