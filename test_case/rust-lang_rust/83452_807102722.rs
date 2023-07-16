rust
// Say this is just a regular inherent struct method
pub const fn get_usize(self) -> usize {
    NonSelfConcreteImplementerOfSomeTrait::USIZE
}
