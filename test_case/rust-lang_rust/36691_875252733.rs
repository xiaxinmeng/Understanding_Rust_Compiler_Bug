
error[E0583]: file not found for module `foo`
 --> src/main.rs:3:10
  |
3 | #[derive(foo)]
  |          ^^^
  |
  = help: to create the module `foo`, create file "src/foo.rs" or "src/foo/mod.rs"
  = note: this error originates in the derive macro `foo` (in Nightly builds, run with -Z macro-backtrace for more info)
