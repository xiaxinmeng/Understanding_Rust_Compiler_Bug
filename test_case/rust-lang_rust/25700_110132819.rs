 Rust
pub trait Parser {
    type Input;
}

pub struct Iter<P: Parser>(P::Input);

pub struct Map<P, F>(P, F);
impl<P, F> Parser for Map<P, F> where F: FnMut(P) {
    type Input = ();
}

trait AstId { type Untyped; }
impl AstId for u8 { type Untyped = u8; }

fn foo<P: Parser>(_: P, h: P::Input) {
    Iter::<P>(h);
}

fn record_type<Id: AstId>(i: Id::Untyped) {
    foo(Map(i, |_: Id::Untyped| { }), ())
}

pub fn main() {
    record_type::<u8>(3);
}
