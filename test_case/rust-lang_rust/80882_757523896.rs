rust
/// The `Bar` trait.
///
/// - Here is another trait: [`Foo`]
/// - Here is our similarly-named member fn: [`foo`]
///
/// [`foo`]: Bar::foo
///
pub trait Bar {
    /// The fn `foo` from the trait `Bar`
    fn foo();
}

/// This is the `Foo` trait.
pub trait Foo {}
