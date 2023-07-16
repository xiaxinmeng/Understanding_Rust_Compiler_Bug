rust
struct Foo<'a, T>(&'a mut T, &'a mut T);
let foo = Foo { ... };

fn before<'a, T>(x: Foo<'a, T>) { ...x.0/x.1 aren't noalias... }
before(foo);

fn after<'a, T>(x0: &'a mut T, x1: &'a mut T) { ...x0,x1 are noalias... }
let Foo(x0, x1) = foo;
after(x0, x1);
