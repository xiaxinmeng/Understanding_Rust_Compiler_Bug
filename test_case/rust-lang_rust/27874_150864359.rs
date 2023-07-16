
foo.rs:3:19: 3:29 error: the trait `Hack` cannot be made into an object [E0038]
foo.rs:3 fn takes_hack(x: &Hack<S=()>) {}
                           ^~~~~~~~~~
foo.rs:3:19: 3:29 help: run `rustc --explain E0038` to see a detailed explanation
foo.rs:3:19: 3:29 note: the trait cannot use `Self` as a type parameter in the supertrait listing
