
#![feature(dyn_trait)]
#![feature(conservative_impl_trait)]
#![feature(underscore_lifetimes)]

fn a<'a, T>(items: &'a [T]) -> Box<impl Iterator> {
    Box::new(items.iter())
}

fn main() { }
