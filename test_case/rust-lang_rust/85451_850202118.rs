rust
pub trait Container {
    type Item;
}

pub trait HasType {
    type Type;
}

/// Take a container with Item = &(H::Type,). Fails
fn foo<'a, H, C>(_: C)
where
    H: HasType,
    C: Container<Item = &'a (H::Type,)>,
    (H::Type,): 'a,
{}
