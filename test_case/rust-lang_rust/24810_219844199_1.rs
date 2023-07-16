
iss_24810_2.rs:6:12: 6:23 error: the trait `T` cannot be made into an object [E0038]
iss_24810_2.rs:6 impl<X: T> T<Output=X> {
                            ^~~~~~~~~~~
iss_24810_2.rs:6:12: 6:23 help: run `rustc --explain E0038` to see a detailed explanation
iss_24810_2.rs:6:12: 6:23 note: the trait cannot use `Self` as a type parameter in the supertrait listing
iss_24810_2.rs:6:6: 6:7 error: the type parameter `X` is not constrained by the impl trait, self type, or predicates [E0207]
iss_24810_2.rs:6 impl<X: T> T<Output=X> {
                      ^
iss_24810_2.rs:6:6: 6:7 help: run `rustc --explain E0207` to see a detailed explanation
error: aborting due to 2 previous errors
