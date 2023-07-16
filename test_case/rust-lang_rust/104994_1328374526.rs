plain
    Checking tracing v0.1.35
    Checking thiserror v1.0.33
    Checking tracing-subscriber v0.3.3
    Checking tracing-tree v0.2.0
    Checking lol_html v0.2.0
    Checking rustdoc-json-types v0.1.0 (/checkout/src/rustdoc-json-types)
    Checking rustdoc-json-types v0.1.0 (/checkout/src/rustdoc-json-types)
    Checking cargo-deadlinks v0.8.0 (https://github.com/jyn514/cargo-deadlinks/?branch=http-feature#6eb4eb1a)
   Compiling askama_derive v0.11.0
    Checking askama v0.11.0
    Checking rustdoc v0.0.0 (/checkout/src/librustdoc)
    Checking rustdoc-tool v0.0.0 (/checkout/src/tools/rustdoc)
---
configure: rust.debug-assertions := True
configure: rust.overflow-checks := True
configure: llvm.assertions      := True
configure: dist.missing-tools   := True
configure: build.configure-args := ['--enable-sccache', '--disable-manage-submodu ...
configure: writing `config.toml` in current directory
configure: 
configure: run `python /checkout/x.py --help`
Attempting with retry: make prepare
---
    Checking thiserror v1.0.33
    Checking tracing v0.1.35
    Checking tracing-subscriber v0.3.3
    Checking tracing-tree v0.2.0
    Checking lol_html v0.2.0
    Checking toml v0.5.7
    Checking cargo-deadlinks v0.8.0 (https://github.com/jyn514/cargo-deadlinks/?branch=http-feature#6eb4eb1a)
    Checking askama_shared v0.12.0
   Compiling askama_derive v0.11.0
    Checking askama v0.11.0
    Checking rustdoc v0.0.0 (/checkout/src/librustdoc)
---
   Compiling ignore v0.4.18
   Compiling tidy v0.1.0 (/checkout/src/tools/tidy)
    Finished release [optimized] target(s) in 6.88s
tidy check
tidy error: invalid source: "git+https://github.com/jyn514/cargo-deadlinks/?branch=http-feature#6eb4eb1a6721a12e131b839e0d67686a9339e1d6"
Checking which error codes lack tests...
* 632 error codes
* highest error code: E0790
Found 506 error codes
Found 506 error codes
Found 0 error(s) in error codes
Done!
tidy error: invalid license `BSD-2-Clause` in `cloudabi 0.0.3 (registry+https://github.com/rust-lang/crates.io-index)`
tidy error: invalid license `MPL-2.0` in `cssparser 0.25.9 (registry+https://github.com/rust-lang/crates.io-index)`
tidy error: invalid license `MPL-2.0` in `cssparser-macros 0.3.6 (registry+https://github.com/rust-lang/crates.io-index)`
tidy error: invalid license `MPL-2.0` in `dtoa-short 0.3.3 (registry+https://github.com/rust-lang/crates.io-index)`
tidy error: invalid license `(Apache-2.0 OR MIT) AND BSD-3-Clause` in `encoding_rs 0.8.31 (registry+https://github.com/rust-lang/crates.io-index)`
tidy error: dependency `fuchsia-cprng 0.1.1 (registry+https://github.com/rust-lang/crates.io-index)` does not define a license expression
tidy error: invalid license `BSD-3-Clause` in `lol_html 0.2.0 (registry+https://github.com/rust-lang/crates.io-index)`
tidy error: invalid license `MPL-2.0` in `selectors 0.21.0 (registry+https://github.com/rust-lang/crates.io-index)`
tidy error: invalid license `MPL-2.0` in `thin-slice 0.1.1 (registry+https://github.com/rust-lang/crates.io-index)`
tidy error: /checkout/src/librustdoc/html/render/context.rs:183: TODO is deprecated; use FIXME
some tidy checks failed
Build completed unsuccessfully in 0:00:09
