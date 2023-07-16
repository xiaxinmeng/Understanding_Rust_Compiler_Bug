rust
#![feature(generic_associated_types)]

trait X<'a> {
    type Y<'b>;
    // when I add the following line,
    // it is required that 'b outlives 'a
    fn foo(&self) -> Self::Y<'static>;
    // why?
}
