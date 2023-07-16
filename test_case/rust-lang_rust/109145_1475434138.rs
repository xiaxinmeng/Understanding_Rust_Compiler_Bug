rust
// #![no_std]
use crate::chumsky::{just, Parser, State};

mod chumsky {
    pub struct MapWithState;

    pub trait ParserExtra {
        type State;
    }

    pub struct State<S>(core::marker::PhantomData<(S,)>);

    impl<S> ParserExtra for State<S> {
        type State = S;
    }

    pub struct Just<E>(core::marker::PhantomData<(E,)>);

    pub const fn just<E>() -> Just<E> {
        loop {}
    }

    impl<E> ParserSealed<E> for Just<E> where E: ParserExtra {}

    pub trait ParserSealed<E> {}

    pub trait Parser<E: ParserExtra> {
        fn map_with_state<U, F: Fn(&mut E::State) -> U>(self, f: F)
        where
            Self: Sized,
        {
        }
    }

    impl<E, P> Parser<E> for P
    where
        E: ParserExtra,
        P: ParserSealed<E>,
    {
    }
}

fn main() {
    struct Q<'a>(&'a ());

    just::<State<Q>>(()).map_with_state(|state: Q| loop {});
}

