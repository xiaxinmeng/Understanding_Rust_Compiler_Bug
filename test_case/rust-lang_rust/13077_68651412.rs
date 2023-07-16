
$ rustc main.rs -L .
main.rs:3:27: 3:35 error: cannot refer to the interior of another static, use a constant instead
main.rs:3 static X: &'static int = &ARRAY[0];
                                    ^~~~~~~~
error: aborting due to previous error
$ rustc --verbose --version
rustc 0.13.0-nightly (c6c786671 2015-01-04 00:50:59 +0000)
binary: rustc
commit-hash: c6c786671d692d7b13c2e5c68a53001327b4b125
commit-date: 2015-01-04 00:50:59 +0000
host: x86_64-apple-darwin
release: 0.13.0-nightly
