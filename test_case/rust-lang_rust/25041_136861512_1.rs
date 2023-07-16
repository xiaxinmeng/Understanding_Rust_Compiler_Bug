
<anon>:7:9: 7:10 error: the type parameter `A` is not constrained by the impl trait, self type, or predicates [E0207]
<anon>:7 impl<F, A> Foo for F where F: MyFn<A, Out=()> {}
                 ^
<anon>:7:9: 7:10 help: see the detailed explanation for E0207
error: aborting due to previous error
playpen: application terminated with error code 101
