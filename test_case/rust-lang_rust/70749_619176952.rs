rust
#![crate_type = "lib"]
#![allow(unused)]

pub fn foo() -> Box<dyn Iterator<Item = u32>> {
    use std::iter::empty;

    Box::new(
        empty()
            .my_chain(empty())
            .my_chain(empty())
            .my_chain(empty())
            .my_chain(empty())
            .my_chain(empty())
            .my_chain(empty())
            .my_chain(empty())
            .my_chain(empty())
            .my_chain(empty())
            .my_chain(empty()) // 10th
            .my_chain(empty())
            .my_chain(empty())
            .my_chain(empty())
            .my_chain(empty())
            .my_chain(empty())
            .my_chain(empty()), // 16th
    )
}

// The `Chain` implementation changed in #70896, which stopped reproducing the optimization hang,
// but here's a reduced copy of the old `Chain` for posterity.
struct Chain<A, B> {
    a: A,
    b: B,
    state: ChainState,
}

enum ChainState {
    Both,
    Front,
    Back,
}

impl<A, B> Iterator for Chain<A, B>
where
    A: Iterator,
    B: Iterator<Item = A::Item>,
{
    type Item = A::Item;

    #[inline]
    fn next(&mut self) -> Option<A::Item> {
        match self.state {
            ChainState::Both => match self.a.next() {
                elt @ Some(..) => elt,
                None => {
                    self.state = ChainState::Back;
                    self.b.next()
                }
            },
            ChainState::Front => self.a.next(),
            ChainState::Back => self.b.next(),
        }
    }
}

trait IteratorExt: Iterator {
    fn my_chain<U>(self, other: U) -> Chain<Self, U::IntoIter>
    where
        Self: Sized,
        U: IntoIterator<Item = Self::Item>,
    {
        Chain {
            a: self,
            b: other.into_iter(),
            state: ChainState::Both,
        }
    }
}

impl<I: Iterator> IteratorExt for I {}
