
// you write
struct Foo { ... }
// compiler writes
impl Reflect for Foo { ... }

// you write
struct Bar<T> { ... }
// compiler writes
impl<T: Reflect> Reflect for Foo<T> { ... }

// you write
struct Baz<'a, T> { ... }
// compiler writes
impl<T: Reflect> Reflect for Baz<'static, T> { ... }
