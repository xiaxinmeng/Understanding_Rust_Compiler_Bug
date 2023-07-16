
$ cargo +nightly-2020-05-28 doc
 Documenting doctest v0.1.0 (/home/daboross/doctest)
warning: `[SpecialFoo::foo]` cannot be resolved, ignoring it.
 --> src/lib.rs:1:29
  |
1 | //! Check out the docs for [`SpecialFoo::foo`].
  |                             ^^^^^^^^^^^^^^^^^ cannot be resolved, ignoring
  |
  = note: `#[warn(intra_doc_link_resolution_failure)]` on by default
  = help: to escape `[` and `]` characters, just add '\' before them like `\[` or `\]`

warning: `[SpecialFoo::foo]` cannot be resolved, ignoring it.
 --> src/lib.rs:5:13
  |
5 |    /// See [`SpecialFoo::foo`] for an example.
  |             ^^^^^^^^^^^^^^^^^ cannot be resolved, ignoring
  |
  = help: to escape `[` and `]` characters, just add '\' before them like `\[` or `\]`

warning: 2 warnings emitted

    Finished dev [unoptimized + debuginfo] target(s) in 0.70s
$ cargo +nightly-2020-05-28 --version
cargo 1.45.0-nightly (9fcb8c1d2 2020-05-25)
$ rustc +nightly-2020-05-28 --version
rustc 1.45.0-nightly (664fcd3f0 2020-05-27)
$ rustdoc +nightly-2020-05-28 --version
rustdoc 1.45.0-nightly (664fcd3f0 2020-05-27)
