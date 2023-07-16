
2020-03-04T04:08:54.5742630Z duplicate artifacts found when compiling a tool, this typically means that something was recompiled because a transitive dependency has different features activated than in a previous build:
2020-03-04T04:08:54.5743350Z 
2020-03-04T04:08:54.5743800Z the following dependencies are duplicated although they have the same features enabled:
2020-03-04T04:08:54.5744250Z the following dependencies have different features:
2020-03-04T04:08:54.5745550Z   proc-macro2 0.4.30 (registry+https://github.com/rust-lang/crates.io-index)
2020-03-04T04:08:54.5746790Z     `rustfmt` additionally enabled features {"default"} at "/Users/runner/runners/2.165.0/work/1/s/build/x86_64-apple-darwin/stage1-tools/x86_64-apple-darwin/release/deps/libproc_macro2-58288f9f59724080.rlib"
2020-03-04T04:08:54.5748230Z     `cargo` additionally enabled features {} at "/Users/runner/runners/2.165.0/work/1/s/build/x86_64-apple-darwin/stage1-tools/x86_64-apple-darwin/release/deps/libproc_macro2-ad9101f6f50fce55.rlib"
2020-03-04T04:08:54.5749280Z   quote 0.6.12 (registry+https://github.com/rust-lang/crates.io-index)
2020-03-04T04:08:54.5750460Z     `rustfmt` additionally enabled features {"default"} at "/Users/runner/runners/2.165.0/work/1/s/build/x86_64-apple-darwin/stage1-tools/x86_64-apple-darwin/release/deps/libquote-d6c8c62334473aa9.rlib"
2020-03-04T04:08:54.5751860Z     `cargo` additionally enabled features {} at "/Users/runner/runners/2.165.0/work/1/s/build/x86_64-apple-darwin/stage1-tools/x86_64-apple-darwin/release/deps/libquote-571fc6c253801dac.rlib"
2020-03-04T04:08:54.5752910Z   syn 0.15.35 (registry+https://github.com/rust-lang/crates.io-index)
2020-03-04T04:08:54.5754070Z     `rustfmt` additionally enabled features {"visit"} at "/Users/runner/runners/2.165.0/work/1/s/build/x86_64-apple-darwin/stage1-tools/x86_64-apple-darwin/release/deps/libsyn-7a2f4a52e285b5df.rlib"
2020-03-04T04:08:54.5756090Z     `cargo` additionally enabled features {} at "/Users/runner/runners/2.165.0/work/1/s/build/x86_64-apple-darwin/stage1-tools/x86_64-apple-darwin/release/deps/libsyn-61f655d09c570865.rlib"
