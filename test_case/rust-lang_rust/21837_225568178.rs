
<anon>:2:16: 2:17 error: parameter `T` is never used [E0392]
<anon>:2 pub struct Foo<T: Bound>;
                        ^
<anon>:2:16: 2:17 help: see the detailed explanation for E0392
<anon>:2:16: 2:17 help: consider removing `T` or using a marker such as `core::marker::PhantomData`
<anon>:8:9: 8:15 error: the trait `Bound` is not implemented for the type `T` [E0277]
<anon>:8 impl<T> Trait2 for Foo<T> { }
                 ^~~~~~
<anon>:8:9: 8:15 help: see the detailed explanation for E0277
<anon>:8:9: 8:15 note: required by `Foo`
error: aborting due to 2 previous errors
