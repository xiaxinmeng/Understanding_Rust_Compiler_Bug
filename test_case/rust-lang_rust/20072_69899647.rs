 sh
$ rustc foo.rs
foo.rs:2:30: 2:33 error: explicit lifetime bound required
foo.rs:2     fn bar(baz: Self) -> Box<Ord>;
                                      ^~~
