 rust
use std::ops::Shl;

pub trait Subscriber {
    type Input;
}

pub trait Publisher<'a> {
    type Output;
}

pub trait Processor<'a> : Subscriber + Publisher<'a> { }

impl<'a, P> Processor<'a> for P
where P : Subscriber + Publisher<'a> { }


impl<'a, PR, PB> Shl<Box<PR>> for Box<PB>
where PR : Processor<'a, Input=<PB as Publisher<'a>>::Output>,
      PB : Publisher<'a, Output=<PR as Processor<'a>>::Input>
{
    type Output = Box<Publisher<'a, Output=<PR as Processor<'a>>::Output> + 'a>;

    fn shl(self, rhs: Box<PR>) -> Box<<Self as Shl<Box<PR>>>::Output> {
        rhs as Box<PB>
    }
}


fn main() {}
