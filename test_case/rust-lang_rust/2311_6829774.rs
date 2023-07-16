 rust
iface clam<A> { }
class foo<A> {
   new() {}
   fn bar<B,C:clam<A>>(c: C) -> B { 1 }
}

fn main() { }
