rust
#![feature(generic_associated_types)]

trait Monad {
    type Unplug;
    type Plug<B>: Monad<Unplug = B>;

    fn plug(t: Self::Unplug) -> Self;

    fn bind<B, F>(self, f: F) -> Self::Plug<B>
    where
        F: Fn(Self::Unplug) -> Self::Plug<B>;
}

impl<A> Monad for Option<A> {
    type Unplug = A;
    type Plug<B> = Option<B>;
    
    fn plug(t: Self::Unplug) -> Self {
        Some(t)
    }
    
    fn bind<B, F>(self, f: F) -> Option<B>
    where
        F: Fn(A) -> Option<B>,
    {
        self.and_then(f)
    }
}

fn stringify<T, M1>(m: M1) -> <M1 as Monad>::Plug<String>
where
    T: core::fmt::Display,
    M1: Monad<Unplug = T>,
{
    m.bind(|x| M1::Plug::plug(format!("{}", x)))
}
