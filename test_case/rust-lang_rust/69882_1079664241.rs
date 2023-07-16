rust
trait Trait {}

fn magic() -> impl Trait {
    panic!()
}
