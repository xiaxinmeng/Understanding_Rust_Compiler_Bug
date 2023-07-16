
foo.rs:9:12: 9:13 error: the type parameter `U` is not constrained by the impl trait, self type, or predicates [E0207]
foo.rs:9 impl<From, U, Added, T: Append<U, Result = Added>> Writer<From, T> {
                    ^
foo.rs:9:12: 9:13 help: run `rustc --explain E0207` to see a detailed explanation
foo.rs:9:15: 9:20 error: the type parameter `Added` is not constrained by the impl trait, self type, or predicates [E0207]
foo.rs:9 impl<From, U, Added, T: Append<U, Result = Added>> Writer<From, T> {
                       ^~~~~
foo.rs:9:15: 9:20 help: run `rustc --explain E0207` to see a detailed explanation
error: aborting due to 2 previous errors
