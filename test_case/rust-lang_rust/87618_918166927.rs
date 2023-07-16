plain
configure: rust.channel         := nightly
configure: rust.debug-assertions := True
configure: llvm.assertions      := True
configure: dist.missing-tools   := True
configure: build.configure-args := ['--enable-sccache', '--disable-manage-submodu ...
configure: writing `config.toml` in current directory
configure: 
configure: run `python /checkout/x.py --help`
configure: 
---
Building stage0 std artifacts (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
    Finished release [optimized] target(s) in 0.15s
Copying stage0 std from stage0 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu / x86_64-unknown-linux-gnu)
 Documenting core v0.0.0 (/checkout/library/core)
error: `char#tymethod.from_str` contains an anchor, but links to builtin types are already anchored
    |
    |
197 | /// This `struct` is created when using the [`char::from_str`](char#tymethod.from_str) method.
    |                                                                    |
    |                                                                    invalid anchor
    |
    |
    = note: `-D rustdoc::broken-intra-doc-links` implied by `-D warnings`
    = note: this restriction may be lifted in a future release
    = note: see https://github.com/rust-lang/rust/issues/83083 for more information

error: `char#impl-TryFrom<u32>` contains an anchor, but links to builtin types are already anchored
    |
    |
264 | /// This `struct` is created by the [`char::try_from<u32>`](char#impl-TryFrom<u32>) method.
    |                                                                 |
    |                                                                 invalid anchor
    |
    = note: this restriction may be lifted in a future release
    = note: this restriction may be lifted in a future release
    = note: see https://github.com/rust-lang/rust/issues/83083 for more information

error: `char#tymethod.from_str` contains an anchor, but links to builtin types are already anchored
    |
    |
197 | /// This `struct` is created when using the [`char::from_str`](char#tymethod.from_str) method.
    |                                                                    |
    |                                                                    invalid anchor
    |
    = note: this restriction may be lifted in a future release
    = note: this restriction may be lifted in a future release
    = note: see https://github.com/rust-lang/rust/issues/83083 for more information

error: could not document `core`

Caused by:
  process didn't exit successfully: `/checkout/obj/build/bootstrap/debug/rustdoc --edition=2018 --crate-type lib --crate-name core library/core/src/lib.rs --target x86_64-unknown-linux-gnu -o /checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/doc --error-format=json --json=diagnostic-rendered-ansi --markdown-css rust.css --markdown-no-toc -Z unstable-options --resource-suffix 1.57.0 --index-page /checkout/src/doc/index.md -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/release/deps --cfg=bootstrap -Dwarnings '-Wrustdoc::invalid_codeblock_attributes' --crate-version '1.57.0-nightly
  (3704eeca8
  2021-09-13)' '-Zcrate-attr=doc(html_root_url="https://doc.rust-lang.org/nightly/")'` (exit status: 1)


command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustdoc" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "16" "--release" "--locked" "--color" "always" "--features" "panic-unwind backtrace compiler-builtins-c" "--manifest-path" "/checkout/library/test/Cargo.toml" "-p" "core" "-Zskip-rustdoc-fingerprint" "--" "--markdown-css" "rust.css" "--markdown-no-toc" "-Z" "unstable-options" "--resource-suffix" "1.57.0" "--index-page" "/checkout/src/doc/index.md"


Build completed unsuccessfully in 0:00:06
