 rust
struct Foo<X: TraitDoesntExist>;
enum Bar<X: TraitDoesntExist> {}

trait Trait<T> {}

struct Baz<X: Trait<Invalid>>;
enum Qux<X: Trait<Invalid>> {}

trait Trait2<'a> {}

struct Baz2<X: Trait<'a>>;
enum Quz2<X: Trait<'a>>;

fn main() {}
