 Rust
use std::marker::PhantomData;

pub trait ObligationProcessor {
    type Obligation;

    // FIXME: crazy lifetime troubles
    fn process_backedge<'a, I>(&mut self, _mkr: PhantomData<&'a Self::Obligation>)
        where I: Iterator<Item=&'a Self::Obligation>;
}

impl<'tcx> ObligationProcessor for &'tcx () {
    type Obligation = &'tcx ();

    fn process_backedge<'c, I>(&mut self, _mkr: PhantomData<&'c &'tcx ()>)
        where I: Iterator<Item=&'c Self::Obligation>
    {
        loop {}
    }
}

fn main() {}
