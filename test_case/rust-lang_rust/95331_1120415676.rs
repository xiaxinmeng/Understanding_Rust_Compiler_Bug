rust
#![feature(generic_associated_types)]

trait FooBad {
    type Item<'a>;
    fn items<'b>(walker: &'b mut dyn for<'a> FnMut(Self::Item<'a>))
    where
        Self: 'b;
}

// or, if the method recieves `&self`, make use of the implied bound `Self: 'b`
trait FooBadWithSelf {
    type Item<'a>;
    fn items<'b>(&'b self, walker: &'b mut dyn for<'a> FnMut(Self::Item<'a>));
}
