
hello.rs:5:40: 5:49 error: unsupported cyclic reference between types/traits detected [E0391]
hello.rs:5 trait Stream<T>: Future<Option<(T, Box<Stream<T>>)>> {
                                                  ^~~~~~~~~
hello.rs:5:40: 5:49 help: run `rustc --explain E0391` to see a detailed explanation
note: the cycle begins when computing the supertraits of `Stream`...
note: ...which then again requires computing the supertraits of `Stream`, completing the cycle.
hello.rs:5:40: 5:49 error: unsupported cyclic reference between types/traits detected [E0391]
hello.rs:5 trait Stream<T>: Future<Option<(T, Box<Stream<T>>)>> {
                                                  ^~~~~~~~~
hello.rs:5:40: 5:49 help: run `rustc --explain E0391` to see a detailed explanation
note: the cycle begins when computing the supertraits of `Stream`...
note: ...which then again requires computing the supertraits of `Stream`, completing the cycle.
error: aborting due to 2 previous errors

