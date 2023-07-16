
pub struct C<AType: A> {a:AType}

pub trait A {
    type B = C<Self::anything_here_kills_it>;
}

fn main(){}
