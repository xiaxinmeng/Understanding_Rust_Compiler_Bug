rust
#![feature(arbitrary_self_types, pin)]

use std::marker::Unpin;
use std::mem::PinMut;

trait Future {
    fn poll(self: PinMut<Self>);
}

trait FutureExt: Future {
    fn poll_unpin(&mut self) where Self: Unpin {
        PinMut::new(self).poll()
    }
}
