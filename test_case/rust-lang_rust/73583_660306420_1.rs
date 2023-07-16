rust
[src/main.rs:3] "foo-bar" < "foo/bar" = true
[src/main.rs:4] Path::new("foo-bar") < Path::new("foo/bar") = false
[src/main.rs:5] vec!["foo-bar"] < vec!["foo", "bar"] = false
