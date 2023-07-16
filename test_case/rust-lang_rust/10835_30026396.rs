
struct Foo
{
c: SharedChan<Foo>,
p: Port<Foo>
}

fn leak_memory()
{
let (p, c) = SharedChan::new();
let x = Foo {p: p, c: c.clone()};
c.send(x);
}
