
$ rustc test.rs
test.rs:5:11: 5:22 error: no method named `to_string` found for type `<Self as Thing<'a>>::X` in the current scope
test.rs:5         x.to_string()
                    ^~~~~~~~~~~
test.rs:5:11: 5:22 note: the method `to_string` exists but the following trait bounds were not satisfied: `<Self as Thing<'_>>::X : core::fmt::Display`
error: aborting due to previous error
