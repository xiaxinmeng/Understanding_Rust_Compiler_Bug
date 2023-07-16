rust
#![warn(keyword_idents)]

pub trait Trait { }

impl Trait for &'static str { }

macro_rules! m { () => { dyn Trait } }

fn foo() -> Box<m!()> { Box::new("from foo") }

fn bar() -> Box<dyn Trait> { Box::new("from bar") }

fn main() { let _ = (foo(), bar()); }
