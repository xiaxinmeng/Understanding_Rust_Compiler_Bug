Rust
#![feature(let_chains)]

struct Pd;

impl Pd {
    fn it(&self) -> It {
        todo!()
    }
}

pub struct It<'a>(Box<dyn Tr<'a>>);

trait Tr<'a> {}

fn f(m: Option<Pd>) {
    if let Some(n) = m && let it = n.iter() {};
}
