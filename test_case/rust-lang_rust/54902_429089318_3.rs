rust
trait WithType<T> {}
trait WithRegion<'a> { }

trait Foo { }

impl<T> Foo for Vec<T>
where T: WithRegion<'_> { }

fn main() {}
