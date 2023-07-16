rust
trait Trait {}
async fn f<'a>(_a: &(), _b: Box<dyn Trait>) {}
