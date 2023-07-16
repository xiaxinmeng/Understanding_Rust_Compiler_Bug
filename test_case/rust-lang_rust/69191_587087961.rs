rust
#![allow(deprecated)]
#![feature(never_type)]

extern crate mio;

use mio::deprecated::EventLoop;

pub trait Machine: Sized {
    type Context;
    type Seed: Sized;

    fn create(seed: Self::Seed) -> Response<Self, ()>;
}

pub struct Response<M, N>(std::marker::PhantomData<(M, N)>);

pub struct Loop<M: Machine> {
    mio: EventLoop<Handler<M>>,
    handler: Handler<M>,
}

impl<M: Machine> Loop<M> {
    pub fn new() -> Self {
        todo!()
    }

    pub fn add_machine_with<F>(&mut self, _fun: F)
    where
        F: FnOnce() -> Response<M, !>,
    {
        todo!()
    }

    pub fn run(mut self) {
        let ref mut handler = self.handler;
        let ref mut mio = self.mio;
        mio.run(handler).unwrap();
    }
}

pub struct Handler<M: Machine> {
    _a: std::marker::PhantomData<M>,
}

impl<M: Machine> Handler<M> {
    pub fn new() -> Self {
        todo!()
    }
}

pub fn machine_loop<M>(_handler: &mut Handler<M>)
where
    M: Machine,
{
    let creator = None;
    let mut new = Some(creator.unwrap());
    let _mach = decompose(M::create(new.take().unwrap()));
}

impl<M: Machine> mio::deprecated::Handler for Handler<M> {
    type Message = ();
    type Timeout = ();
    fn timeout(&mut self, _eloop: &mut EventLoop<Self>, _: ()) {
        machine_loop(self)
    }
}

impl<M: Sized, N: Sized> Response<M, N> {
    pub fn map<T, U, S, R>(self, _: S, _: R) -> Response<T, U>
    where
        S: FnOnce(M) -> T,
    {
        todo!()
    }
    pub fn wrap<T, S>(self, _: S) -> Response<T, N>
    where
        S: FnOnce(M) -> T,
    {
        todo!()
    }
}

pub fn decompose<M, N>(_: Response<M, N>) -> M {
    todo!()
}

enum Tcp {}

enum Composed {
    Tcp(Tcp),
}
enum CSeed {
    Tcp(!),
}

impl Machine for Composed {
    type Context = u32;
    type Seed = CSeed;
    fn create(seed: CSeed) -> Response<Self, ()> {
        match seed {
            CSeed::Tcp(x) => todo!(),
        }
    }
}

impl Tcp {
    fn new() -> Response<Tcp, !> {
        todo!()
    }
}

impl Machine for Tcp {
    type Context = u32;
    type Seed = !;
    fn create(_seed: !) -> Response<Self, ()> {
        todo!()
    }
}

fn main() {
    let mut loop_creator = Loop::new();
    loop_creator.add_machine_with(|| Tcp::new().wrap(Composed::Tcp));
    loop_creator.run();
}
