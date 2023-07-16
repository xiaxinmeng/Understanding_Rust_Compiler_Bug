
$ cargo --version
cargo 0.17.0-nightly (740f9c0 2016-12-29)
$ rustc --version
rustc 1.16.0-nightly (4ecc85beb 2016-12-28)
$ cargo build -v
   Compiling parser v0.1.0 (file:///builds/srwalker/rsdev/parser)
     Running `rustc --crate-name parser lib.rs --crate-type lib -g -C metadata=be6ddb31ceb06482 -C extra-filename=-be6ddb31ceb06482 --out-dir /builds/srwalker/rsdev/parser/target/debug/deps --emit=dep-info,link -L dependency=/builds/srwalker/rsdev/parser/target/debug/deps`
warning: constant item is never used: `ACT_STRINGS`, #[warn(dead_code)] on by default
 --> lib.rs:1:1
  |
1 | const ACT_STRINGS: &'static [&'static str] = &["set name=foo value=foo"];
  | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

    Finished debug [unoptimized + debuginfo] target(s) in 0.6 secs
