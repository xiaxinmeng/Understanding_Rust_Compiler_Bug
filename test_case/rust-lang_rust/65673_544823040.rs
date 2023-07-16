rust
trait Trait {}
trait WithType {
    type Ctx;
}
trait Alias<T> = where T: Trait;

impl<T> WithType for T {
    type Ctx = dyn Alias<T>;
}
