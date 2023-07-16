rust
#[fundamental]
trait Closure {
    type Captures/*: Tuple*/;
    fn captures_ref(&self) -> &Self::Captures;
}
struct Capture<const NAME: &'static str, T>(T);
