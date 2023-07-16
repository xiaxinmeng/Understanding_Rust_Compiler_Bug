
mod private {
    pub struct ShallowlyPublic;
}

// Compiles!
// Value of non-exported type is available to other crates -> hello link time errors.
pub fn f() -> private::ShallowlyPublic {
    private::ShallowlyPublic
}
