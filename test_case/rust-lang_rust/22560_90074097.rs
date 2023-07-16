
$ rustc 22560.rs
22560.rs:3:13: 3:16 error: the type parameter `RHS` must be explicitly specified in an object type because its default value `Self` references the type `Self`
22560.rs:3 type Test = Add + Sub;
                     ^~~
22560.rs:3:19: 3:22 error: only the builtin traits can be used as closure or object bounds [E0225]
22560.rs:3 type Test = Add + Sub;
                           ^~~
error: aborting due to 2 previous errors
$ rustc -Vv
rustc 1.0.0-nightly (083b8a404 2015-04-05) (built 2015-04-04)
binary: rustc
commit-hash: 083b8a40413eb3dfec7430150b8895f6c8bbafca
commit-date: 2015-04-05
build-date: 2015-04-04
host: x86_64-apple-darwin
release: 1.0.0-nightly
