
hello.rs:9:6: 9:7 error: the type parameter `A` is not constrained by the impl trait, self type, or predicates [E0207]
hello.rs:9 impl<A: Send, F: Foo<A>> Wrap<F> {
                ^~
hello.rs:9:6: 9:7 help: run `rustc --explain E0207` to see a detailed explanation
error: aborting due to previous error
