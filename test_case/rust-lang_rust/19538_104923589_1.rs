
19538.rs:18:26: 18:36 error: cannot convert to a trait object because trait `Bar` is not object-safe [E0038]
19538.rs:18     let test: &mut Bar = &mut thing;
                                     ^~~~~~~~~~
19538.rs:18:26: 18:36 note: method `foo` has generic type parameters
19538.rs:18     let test: &mut Bar = &mut thing;
                                     ^~~~~~~~~~
error: aborting due to previous error
