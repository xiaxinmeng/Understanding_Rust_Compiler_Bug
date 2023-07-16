 rust
trait Trait { }

fn a(x: &Trait) { try(x); }
fn b(x: &mut Trait) { try(x); }
fn c(x: Box<Trait>) { try(x); }
fn try<T: Trait>(t: T) { }

fn main() { }
