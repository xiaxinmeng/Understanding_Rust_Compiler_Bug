
hello.rs:2:6: 2:7 error: the type parameter `A` is not constrained by the impl trait, self type, or predicates [E0207]
hello.rs:2 impl<A: std::fmt::Display, B> Foo {
                ^
hello.rs:2:28: 2:29 error: the type parameter `B` is not constrained by the impl trait, self type, or predicates [E0207]
hello.rs:2 impl<A: std::fmt::Display, B> Foo {
                                      ^
