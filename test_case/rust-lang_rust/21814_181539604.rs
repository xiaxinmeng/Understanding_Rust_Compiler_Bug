
hello.rs:13:15: 13:22 error: the value of the associated type `A` (from the trait `T`) must be specified [E0191]
hello.rs:13   let _: Vec<<Erasure as T>::A> = vec![42u32];
                          ^~~~~~~
hello.rs:13:15: 13:22 help: run `rustc --explain E0191` to see a detailed explanation
hello.rs:13:40: 13:45 error: mismatched types:
 expected `<Erasure as T>::A`,
    found `u32`
(expected associated type,
    found u32) [E0308]
hello.rs:13   let _: Vec<<Erasure as T>::A> = vec![42u32];
                                                   ^~~~~
hello.rs:13:35: 13:46 note: in this expansion of vec! (defined in <std macros>)
hello.rs:13:40: 13:45 help: run `rustc --explain E0308` to see a detailed explanation
error: aborting due to 2 previous errors
