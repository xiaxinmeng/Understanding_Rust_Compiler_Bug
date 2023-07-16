
$ rustc +nightly-2015-05-13 -Vv
rustc 1.1.0-nightly (c2b30b86d 2015-05-12) (built 2015-05-13)
binary: rustc
commit-hash: c2b30b86df6b34ba19e87e63402e43d9e81a64fb
commit-date: 2015-05-12
build-date: 2015-05-13
host: x86_64-unknown-linux-gnu
release: 1.1.0-nightly

$ rustc +nightly-2015-05-13 issue67369.rs
issue67369.rs:1:1: 3:2 warning: struct is never used: `Foo`, #[warn(dead_code)] on by default
issue67369.rs:1 struct Foo<T: ?Sized> {
issue67369.rs:2     value: T,
issue67369.rs:3 }
issue67369.rs:2:5: 2:13 warning: struct field is never used: `value`, #[warn(dead_code)] on by default
issue67369.rs:2     value: T,
                    ^~~~~~~~
issue67369.rs:11:1: 13:2 warning: struct is never used: `Bar`, #[warn(dead_code)] on by default
issue67369.rs:11 struct Bar {
issue67369.rs:12     ptr: Box<Foo<Trait>>,
issue67369.rs:13 }


$ rustc +nightly-2015-05-14 -Vv
rustc 1.1.0-nightly (e5394240a 2015-05-14) (built 2015-05-14)
binary: rustc
commit-hash: e5394240a295650b567aa406b4a0e1e3a6749a5f
commit-date: 2015-05-14
build-date: 2015-05-14
host: x86_64-unknown-linux-gnu
release: 1.1.0-nightly

$ rustc +nightly-2015-05-14 issue67369.rs
issue67369.rs:17:21: 17:26 error: cannot move out of borrowed content
issue67369.rs:17         self.ptr == other.ptr
                                     ^~~~~
error: aborting due to previous error
