rust
#![feature(generic_associated_types)]

trait Monad {
    type Unplug;
    type Plug<B>: Monad;

    fn bind<B, F>(self, f: F) -> Self::Plug<B>
    where
        F: Fn(Self::Unplug) -> Self::Plug<B>;
}

impl<A> Monad for Option<A> {
    type Unplug = A;
    type Plug<B> = Option<B>;
    
    fn bind<B, F>(self, f: F) -> Option<B>
    where
        F: Fn(A) -> Option<B>,
    {
        self.and_then(f)
    }
}

impl<A> Monad for Result<A, ()> {
    type Unplug = A;
    type Plug<B> = Result<B, ()>;
    
    fn bind<B, F>(self, f: F) -> Result<B, ()>
    where
        F: Fn(A) -> Result<B, ()>
    {
        self.and_then(f)
    }
}

fn stringify<T, M1>(m: M1) -> <M1 as Monad>::Plug<String>
where
    T: core::fmt::Display,
    M1: Monad<Unplug = T>,
{
    m.bind(|x| Some(format!("{}", x)))
}

fn main() {
    let a: Result<i32, ()> = Ok(0);
    stringify(a); // In this case <M1 as Monad>::Plug<String> = Result<String, ()>`, not `Option<String>`!
}
