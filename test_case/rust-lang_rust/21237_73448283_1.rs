 rust
struct Foo<T>;
impl<T> Debug for Foo<T> where T: Debug { ... }

struct Bar<T>(Foo<T>);
impl<T> Debug for Bar<T> where T: Debug { ... }

trait Trait { type Type; }

struct Baz<T: Trait>(T::Type);
impl<T> Debug for Baz<T> where T: Debug, T::Type: Debug { ... }
