rust
#![feature(generic_associated_types)]

trait MyTrait {
    type AT<'a> where Self: 'a;
    
    fn selfreffing<'a>(&'a self) -> Self::AT<'a> { unimplemented!(); }
}

struct Inner;
impl MyTrait for Inner {
    type AT<'b> where Self: 'b = ();
}

struct Outer<'a, T>(&'a T);
impl<'a, T: MyTrait> MyTrait for Outer<'a, T> {
    type AT<'b> = T::AT<'a>;
    // Should recommend type AT<'b> where Self: 'b = T::AT<'a>;
}
