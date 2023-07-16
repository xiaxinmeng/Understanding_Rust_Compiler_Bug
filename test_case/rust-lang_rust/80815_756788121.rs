
$ git clean -xdff
$ nix-shell -p gcc9 binutils cmake ninja openssl pkgconfig python27 --run './x.py build'
Updating only changed submodules
Submodules updated in 0.08 seconds
downloading https://static.rust-lang.org/dist/2020-12-30/rust-std-beta-x86_64-unknown-linux-gnu.tar.gz
################################################################################################################# 100.0%
extracting /sb/d/rust/build/cache/2020-12-30/rust-std-beta-x86_64-unknown-linux-gnu.tar.gz
downloading https://static.rust-lang.org/dist/2020-12-30/rustc-beta-x86_64-unknown-linux-gnu.tar.gz
################################################################################################################# 100.0%
extracting /sb/d/rust/build/cache/2020-12-30/rustc-beta-x86_64-unknown-linux-gnu.tar.gz
downloading https://static.rust-lang.org/dist/2020-12-30/cargo-beta-x86_64-unknown-linux-gnu.tar.gz
################################################################################################################# 100.0%
extracting /sb/d/rust/build/cache/2020-12-30/cargo-beta-x86_64-unknown-linux-gnu.tar.gz
info: you seem to be running NixOS. Attempting to patch /sb/d/rust/build/x86_64-unknown-linux-gnu/stage0/bin/rustc
info: you seem to be running NixOS. Attempting to patch /sb/d/rust/build/x86_64-unknown-linux-gnu/stage0/bin/rustdoc
info: you seem to be running NixOS. Attempting to patch /sb/d/rust/build/x86_64-unknown-linux-gnu/stage0/bin/cargo
info: you seem to be running NixOS. Attempting to patch /sb/d/rust/build/x86_64-unknown-linux-gnu/stage0/lib/libchalk_derive-568bc6ffcda80739.so
info: you seem to be running NixOS. Attempting to patch /sb/d/rust/build/x86_64-unknown-linux-gnu/stage0/lib/libstd-7444fd17919e9af0.so
info: you seem to be running NixOS. Attempting to patch /sb/d/rust/build/x86_64-unknown-linux-gnu/stage0/lib/librustc_driver-5cca296db1ada831.so
info: you seem to be running NixOS. Attempting to patch /sb/d/rust/build/x86_64-unknown-linux-gnu/stage0/lib/libserde_derive-3c2d48eb0beac455.so
info: you seem to be running NixOS. Attempting to patch /sb/d/rust/build/x86_64-unknown-linux-gnu/stage0/lib/libtest-c99ebba9851136ef.so
info: you seem to be running NixOS. Attempting to patch /sb/d/rust/build/x86_64-unknown-linux-gnu/stage0/lib/libtracing_attributes-51c2223d70bb3634.so
info: you seem to be running NixOS. Attempting to patch /sb/d/rust/build/x86_64-unknown-linux-gnu/stage0/lib/librustc_macros-f9f4f0a9ad2ffc8a.so
info: you seem to be running NixOS. Attempting to patch /sb/d/rust/build/x86_64-unknown-linux-gnu/stage0/lib/libLLVM-11-rust-1.50.0-beta.so
downloading https://static.rust-lang.org/dist/2020-11-19/rustfmt-nightly-x86_64-unknown-linux-gnu.tar.gz
################################################################################################################# 100.0%
extracting /sb/d/rust/build/cache/2020-11-19/rustfmt-nightly-x86_64-unknown-linux-gnu.tar.gz
info: you seem to be running NixOS. Attempting to patch /sb/d/rust/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt
info: you seem to be running NixOS. Attempting to patch /sb/d/rust/build/x86_64-unknown-linux-gnu/stage0/bin/cargo-fmt
   Compiling proc-macro2 v1.0.19
   Compiling unicode-xid v0.2.1
   Compiling syn v1.0.38
   Compiling version_check v0.9.1
   Compiling autocfg v1.0.0
   Compiling memchr v2.3.3
   Compiling serde_derive v1.0.118
   Compiling cfg-if v0.1.10
   Compiling lazy_static v1.4.0
   Compiling libc v0.2.79
   Compiling serde v1.0.118
   Compiling log v0.4.11
   Compiling ryu v1.0.5
   Compiling regex-syntax v0.6.18
   Compiling serde_json v1.0.59
   Compiling fnv v1.0.7
   Compiling same-file v1.0.6
   Compiling unicode-width v0.1.8
   Compiling itoa v0.4.6
   Compiling bootstrap v0.0.0 (/sb/d/rust/src/bootstrap)
   Compiling cc v1.0.60
   Compiling build_helper v0.1.0 (/sb/d/rust/src/build_helper)
   Compiling opener v0.4.1
   Compiling thread_local v1.0.1
   Compiling walkdir v2.3.1
   Compiling getopts v0.2.21
   Compiling proc-macro-error-attr v1.0.4
   Compiling proc-macro-error v1.0.4
   Compiling crossbeam-utils v0.7.2
   Compiling num-traits v0.2.12
   Compiling cmake v0.1.44
   Compiling aho-corasick v0.7.13
   Compiling bstr v0.2.13
   Compiling quote v1.0.7
   Compiling filetime v0.2.12
   Compiling time v0.1.43
   Compiling num_cpus v1.13.0
   Compiling regex v1.3.9
   Compiling globset v0.4.5
   Compiling ignore v0.4.16
   Compiling merge_derive v0.1.0
   Compiling merge v0.1.0
...
