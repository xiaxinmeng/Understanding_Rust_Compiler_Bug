
struct Foo
{
c: SharedChan<Foo>,
p: Port<Foo>
}

fn leak_memory()
{
let (ap, ac) = SharedChan::new();
let (bp, bc) = SharedChan::new();
let a = Foo {p: ap, c: ac.clone()};
let b = Foo {p: bp, c: bc.clone()};
ac.send(b);
bc.send(a);
}
