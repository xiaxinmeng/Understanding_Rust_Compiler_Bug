
trait Foo : Bar<Self> { }
trait Bar<T: Foo> { }
