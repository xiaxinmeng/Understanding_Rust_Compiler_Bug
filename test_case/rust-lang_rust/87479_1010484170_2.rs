rust
#![feature(generic_associated_types)]

trait X<'a> {
    type Y<'b>;
    //type StaticY; // = Y<'static>
    //fn foo(&self) -> Self::StaticY;
    fn foo(&self) -> Self::Y<'static>;
    fn bar<'b>(&self, arg: Self::Y<'b>);
}
