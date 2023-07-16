rust
//#![feature(type_alias_impl_trait)]

trait WithAssoc { type AssocType; }

trait WithParam<A> {}

type Return<A> = impl WithAssoc<AssocType = impl WithParam<A>>;

fn my_fun() -> Return<()> {}

fn main() {}
