:; cargo +nightly doc
 Documenting rustdoc-repro v0.1.0 (/home/eliza/Code/rustdoc-repro)
error: unresolved link to `std::slice::len`
 --> src/lib.rs:6:6
  |
6 | /// [std::slice::len]
  |      ^^^^^^^^^^^^^^^ no item named `len` in module `slice`
  |
note: the lint level is defined here
 --> src/lib.rs:1:9
  |
1 | #![deny(rustdoc::broken_intra_doc_links)]
  |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

error: unresolved link to `std::array::map`
 --> src/lib.rs:7:6
  |
7 | /// [std::array::map]
  |      ^^^^^^^^^^^^^^^ no item named `map` in module `array`

error: unresolved link to `std::tuple`
 --> src/lib.rs:8:6
  |
8 | /// [std::tuple]
  |      ^^^^^^^^^^ no item named `tuple` in module `std`

error: unresolved link to `std::reference`
 --> src/lib.rs:9:6
  |
9 | /// [std::reference]
  |      ^^^^^^^^^^^^^^ no item named `reference` in module `std`

error: could not document `rustdoc-repro`

Caused by:
  process didn't exit successfully: `rustdoc --edition=2018 --crate-type lib --crate-name rustdoc_repro src/lib.rs -o /home/eliza/Code/rustdoc-repro/target/doc --error-format=json --json=diagnostic-rendered-ansi -L dependency=/home/eliza/Code/rustdoc-repro/target/debug/deps --crate-version 0.1.0` (exit status: 1)
