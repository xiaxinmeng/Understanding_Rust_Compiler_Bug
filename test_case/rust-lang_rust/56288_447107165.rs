rust
trait Foo: Bar<Out = Self> {
}

trait Bar { type Out; }
