
Building stage1 tool clippy-driver (armv7-unknown-linux-gnueabihf)
   Compiling cfg-if v0.1.8
   Compiling matches v0.1.8
   Compiling rustc-demangle v0.1.16
   Compiling unicode-normalization v0.1.7
   Compiling unicode-width v0.1.6
   Compiling pulldown-cmark v0.5.3
   Compiling itoa v0.4.4
   Compiling percent-encoding v2.0.0
   Compiling lazy_static v1.3.0
   Compiling ucd-util v0.1.3
   Compiling rustc_tools_util v0.2.0 (/builds/nsidious/rust-ci/rust/src/tools/clippy/rustc_tools_util)
   Compiling quine-mc_cluskey v0.2.4
   Compiling smallvec v0.6.10
   Compiling utf8-ranges v1.0.2
   Compiling if_chain v1.0.0
   Compiling libc v0.2.62
   Compiling ryu v1.0.0
   Compiling unicode-bidi v0.3.4
   Compiling memchr v2.2.0
   Compiling bitflags v1.1.0
   Compiling getopts v0.2.21
   Compiling thread_local v0.3.6
   Compiling regex-syntax v0.6.6
   Compiling regex v1.1.6
   Compiling clippy v0.0.212 (/builds/nsidious/rust-ci/rust/src/tools/clippy)
   Compiling backtrace-sys v0.1.30
   Compiling unicase v2.4.0
   Compiling idna v0.2.0
   Compiling aho-corasick v0.7.3
   Compiling backtrace v0.3.37
   Compiling serde_json v1.0.40
   Compiling toml v0.5.3
   Compiling url v2.1.0
   Compiling failure v0.1.5
   Compiling cargo_metadata v0.8.0
   Compiling clippy_lints v0.0.212 (/builds/nsidious/rust-ci/rust/src/tools/clippy/clippy_lints)
error[E0463]: can't find crate for `fmt_macros`
  --> src/tools/clippy/clippy_lints/src/lib.rs:20:1
   |
20 | extern crate fmt_macros;
   | ^^^^^^^^^^^^^^^^^^^^^^^^ can't find crate

error: aborting due to previous error

For more information about this error, try `rustc --explain E0463`.
error: Could not compile `clippy_lints`.

To learn more, run the command again with --verbose.
