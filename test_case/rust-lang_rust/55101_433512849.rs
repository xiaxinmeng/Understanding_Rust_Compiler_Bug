
trait Foo = Send + Bar;
trait Bar = Sync + Foo;
