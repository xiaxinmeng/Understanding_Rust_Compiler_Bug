console
$ cargo +nightly-2019-01-23 rustdoc
 Documenting rust-orgmode v0.1.0 (/home/misdreavus/clones/rust-orgmode)
error: macro attributes must be placed before `#[derive]`
   --> src/types.rs:337:5
    |
337 |     #[shared_behavior]
    |     ^^^^^^^^^^^^^^^^^^

error: Compilation failed, aborting rustdoc

error: Could not document `rust-orgmode`.

Caused by:
  process didn't exit successfully: `rustdoc --crate-name rust_orgmode src/lib.rs --color always -o /home/misdreavus/clones/rust-orgmode/target/doc -L dependency=/home/misdreavus/clones/rust-orgmode/target/debug/deps --extern chrono=/home/misdreavus/clones/rust-orgmode/target/debug/deps/libchrono-af84f74b14cc1ea4.rmeta --extern failure=/home/misdreavus/clones/rust-orgmode/target/debug/deps/libfailure-13c12b1584de9b63.rmeta --extern failure_derive=/home/misdreavus/clones/rust-orgmode/target/debug/deps/libfailure_derive-4ee6c3df09cefde1.so --extern itertools=/home/misdreavus/clones/rust-orgmode/target/debug/deps/libitertools-1eac39dcb8e2c9f2.rmeta --extern lazy_static=/home/misdreavus/clones/rust-orgmode/target/debug/deps/liblazy_static-41d603f7ffea7a88.rmeta --extern nom=/home/misdreavus/clones/rust-orgmode/target/debug/deps/libnom-77224f728ad9d909.rmeta --extern regex=/home/misdreavus/clones/rust-orgmode/target/debug/deps/libregex-b88256a3f94d06b2.rmeta --extern rust_orgmode_derive=/home/misdreavus/clones/rust-orgmode/target/debug/deps/librust_orgmode_derive-83a3597b783b8ae9.so` (exit code: 1)

$ rustc +nightly-2019-01-23 --version
rustc 1.33.0-nightly (4c2be9c97 2019-01-22)

$ git status
HEAD detached at b1c65e8
nothing to commit, working tree clean
