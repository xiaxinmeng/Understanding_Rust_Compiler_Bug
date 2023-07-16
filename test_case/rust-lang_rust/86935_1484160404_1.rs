
error[[E0658]](https://doc.rust-lang.org/stable/error_codes/E0658.html): usage of qualified paths in this context is experimental
  --> src/main.rs:26:32
   |
26 |             Self::Ref { a } => <Self as EnumRef>::Ref::Ref { a },
   |                                ^^^^^^^^^^^^^^^^^^^^^^^^^^^
