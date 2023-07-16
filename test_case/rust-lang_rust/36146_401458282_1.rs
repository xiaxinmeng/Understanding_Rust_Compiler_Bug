
$ tree src
src
├── a1.rs
├── lib
│   ├── a1.rs
│   └── lib.rs
└── lib.rs
$ cargo +nightly build
   Compiling asdf v0.1.0 (file:///Users/alex/Programming/rust/asdf)
warning: unused attribute
 --> src/lib.rs:1:1
  |
1 | #![feature(non_modrs_mods)]
  | ^^^^^^^^^^^^^^^^^^^^^^^^^^^
  |
  = note: #[warn(unused_attributes)] on by default

warning: crate-level attribute should be in the root module
 --> src/lib.rs:1:1
  |
1 | #![feature(non_modrs_mods)]
  | ^^^^^^^^^^^^^^^^^^^^^^^^^^^

    Finished dev [unoptimized + debuginfo] target(s) in 0.21s
