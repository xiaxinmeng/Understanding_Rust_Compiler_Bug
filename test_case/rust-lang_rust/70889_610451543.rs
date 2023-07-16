rust
trait Foo {}
impl Foo for Phantom<PADDED> {}
impl Foo for Phantom<FILLED> {}
