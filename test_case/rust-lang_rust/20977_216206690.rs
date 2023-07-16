
<anon>:1:25: 1:40 error: wrong number of type arguments: expected 0, found 1 [E0244]
<anon>:1 pub struct Foo { i: Box<Iterator<isize>> }
                                 ^~~~~~~~~~~~~~~
<anon>:1:25: 1:40 help: see the detailed explanation for E0244
<anon>:1:25: 1:40 error: the value of the associated type `Item` (from the trait `core::iter::Iterator`) must be specified [E0191]
<anon>:1 pub struct Foo { i: Box<Iterator<isize>> }
                                 ^~~~~~~~~~~~~~~
<anon>:1:25: 1:40 help: see the detailed explanation for E0191
