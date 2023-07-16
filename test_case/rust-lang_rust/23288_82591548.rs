
/home/alex/code/rust4/src/libcore/slice.rs:1578:9: 1578:10 error: the type parameter `B` is not constrained by the impl trait, self type, or predicates [E0207]
/home/alex/code/rust4/src/libcore/slice.rs:1578 impl<A, B, R: As<[B]>> PartialEq<R> for [A] where A: PartialEq<B> {
                                                        ^
error: aborting due to previous error
