rust
struct Wrapper<T>(T);

trait Base {}
trait Wrapped: Sized where Wrapper<Self>: Base {}

fn wat<F: Wrapped>() where Wrapper<F>: Base {}

fn main() {}
