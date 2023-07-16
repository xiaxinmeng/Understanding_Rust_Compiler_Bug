
#![feature(generic_associated_types)]
#![allow(incomplete_features)]

trait X<'b> {
    type Y<'a: 'b>;
}

struct _S {}

impl<'a> X<'a> for _S {
    type Y<'b: 'a> = ();
}
