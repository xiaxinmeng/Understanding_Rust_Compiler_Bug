rust
trait Container {
    type Item;
}

trait HasType {
    type Type;
}

struct Wrapper<T>(T);

// Take a container with Item = &Wrapper<H::Type>
fn foo<'a, H, C>(_: C)
where
    H: HasType,
    C: Container<Item = &'a Wrapper<H::Type>>,
    Wrapper<H::Type>: 'a,
{}
