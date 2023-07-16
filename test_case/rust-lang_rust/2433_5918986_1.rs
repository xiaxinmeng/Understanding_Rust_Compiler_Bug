
iface A { fn foo() -> bool; }
iface B { fn foo() -> int; }

fn aa<T:A B>(x: T) -> bool { x.foo() }
fn bb<T:B A>(x: T) -> int { x.foo() }

fn ab<T:A B>(x: T) -> int { x.foo() }
fn ba<T:B A>(x: T) -> bool { x.foo() }

fn main() {}
