
trait Reflect<'a> { ... }

struct Foo<'a> { ... }
// auto-derived:
impl Reflect<'a> for Foo<'a> { ... }

type OldReflect = Reflect<'static>

trait Any<'a>: Reflect<'a> { ... }
