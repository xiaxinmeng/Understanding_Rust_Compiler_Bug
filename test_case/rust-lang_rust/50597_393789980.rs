rust
...
// self.extract returns an integer the size of the register.
fn foo(&self) -> bool {
    self.extract(&Self::FIELDS[$id]).try_into().unwrap()
}
fn bar(&self) -> AnEnumThatImplTryFrom {
    self.extract(&Self::FIELDS[$id]).try_into().unwrap()
}
...
