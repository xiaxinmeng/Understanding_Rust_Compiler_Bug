rust
#![feature(generic_associated_types)]
#![feature(associated_type_defaults)]

struct Foo;
struct Bar;
struct Baz;

struct A<'a> {
	foo: &'a Foo,
}

struct B<'a> {
	inherited: A<'a>,
	own: Bar,
}

trait Trait: Clone + Default {
	type Ctx<'a> = A<'a>;
	fn process<'a>(&mut self, ctx: &Self::Ctx/*<'a>*/);
}

impl Trait for Baz {
	type Ctx<'a> = B<'a>;
	fn process<'a>(&mut self, ctx: &Self::Ctx/*<'a>*/) {}
}

fn main() {}
