
iss_24810_1.rs:6:15: 6:16 error: the trait `T` cannot be made into an object [E0038]
iss_24810_1.rs:6 impl T<Output=T> {
                               ^
iss_24810_1.rs:6:15: 6:16 help: run `rustc --explain E0038` to see a detailed explanation
iss_24810_1.rs:6:15: 6:16 note: the trait cannot use `Self` as a type parameter in the supertrait listing
iss_24810_1.rs:6:6: 6:17 error: the trait `T` cannot be made into an object [E0038]
iss_24810_1.rs:6 impl T<Output=T> {
                      ^~~~~~~~~~~
iss_24810_1.rs:6:6: 6:17 help: run `rustc --explain E0038` to see a detailed explanation
iss_24810_1.rs:6:6: 6:17 note: the trait cannot use `Self` as a type parameter in the supertrait listing
error: aborting due to 2 previous errors
