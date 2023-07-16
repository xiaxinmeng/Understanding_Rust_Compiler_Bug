
<anon>:1:1: 3:2 error: recursive type `T` has infinite size [E0072]
<anon>:1 enum Foo {
<anon>:2     Bar(T)
<anon>:3 }
<anon>:1:1: 3:2 help: see the detailed explanation for E0072
<anon>:1:1: 3:2 note: field #0 of the variant `Foo::Bar` has the type `T`
<anon>:1:1: 3:2 note: field `t` of the struct `T` has the type `Foo`, completing the cycle
<anon>:1:1: 3:2 help: insert indirection (e.g., a `Box`, `Rc`, or `&`) at some point to make `T` representable
