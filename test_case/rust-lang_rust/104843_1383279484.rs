rust

#![feature(let_chains)]

struct F(Box<()>);
struct G;
impl Drop for G {
    fn drop(&mut self) {}
}

impl F {
    fn s(&self) -> Option<G> {
        None
    }
}

fn cex() -> Option<F> {
    None
}

pub fn main() {
    if let Some(ce) = cex() && let Some(ce2) = ce.s()
    {
    }
}
