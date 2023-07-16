
$ rustc foo.rs
foo.rs:22:12: 22:15 error: internal compiler error: No variable registered for id 20
foo.rs:22   foo!(def, def);
                      ^~~
foo.rs:8:2: 12:4 note: in expansion of add_foo!
foo.rs:16:6: 16:23 note: expansion site
foo.rs:14:2: 19:4 note: in expansion of foo!
foo.rs:22:2: 22:17 note: expansion site
