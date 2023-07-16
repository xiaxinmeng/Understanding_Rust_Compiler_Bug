
error: cannot find attribute `sede` in this scope
  --> src/main.rs:18:7
   |
18 |     #[sede(untagged)]
   |       ^^^^
   |
help: the derive macros `Serialize` and `Deserialize` accept the similarly named `serde` attribute
   |
18 |     #[serde(untagged)]
   |       ~~~~~

error: cannot find attribute `serde` in this scope
  --> src/main.rs:12:7
   |
12 |     #[serde(untagged)]
   |       ^^^^^
   |
   = note: `serde` is in scope, but it is a crate, not an attribute
help: `serde` is an attribute that can be used by the derive macros `Serialize` and `Deserialize`, you might be missing a `derive` attribute
   |
10 | #[derive(Serialize, Deserialize)]
   |
