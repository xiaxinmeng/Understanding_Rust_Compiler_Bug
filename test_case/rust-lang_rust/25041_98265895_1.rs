
<anon>:2:9: 2:10 error: the type parameter `A` is not constrained by the impl trait, self type, or predicates [E0207]
<anon>:2 impl<F, A, T> Foo for F where F: Fn(A) -> T {}
                 ^
<anon>:2:12: 2:13 error: the type parameter `T` is not constrained by the impl trait, self type, or predicates [E0207]
<anon>:2 impl<F, A, T> Foo for F where F: Fn(A) -> T {}
