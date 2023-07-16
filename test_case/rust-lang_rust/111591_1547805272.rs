Rust
fn main() {
    println!("Hello, world!");
    let a: Box<dyn Protocol<Sx, Output=Sx>> = with_protocols(10, Plain);
}

pub trait Az: 'static {}

impl<A: Az> Az for Box<A> {}

pub struct Sx;

impl Az for Sx {}

pub trait Protocol<I: Az>: 'static {
  type Output: Az;
}

impl<P: Protocol<I, Output=B>, I: Az, B:Az> Protocol<I> for Box<P> {
  type Output=Box<B>;
}

pub struct Plain;

impl<I: Az> Protocol<I> for Plain {
    type Output = I;
}

pub struct JoinProtocol<L, R>(L, R);


impl<L: Protocol<A, Output=B>, R: Protocol<B>, A: Az, B: Az> Protocol<A> for JoinProtocol<L, R> {                        
    type Output = R::Output;                                       
}                                                          


fn with_protocols<A: Az, P: Protocol<A>>(u: u8, p: P) -> Box<dyn Protocol<A, Output=P::Output>> {
  if u == 0 {
    return Box::new(p);
  }
  with_protocols(u-1, JoinProtocol(Plain, p))
}
