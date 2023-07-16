plain
configure: rust.debug-assertions := True
configure: rust.overflow-checks := True
configure: llvm.assertions      := True
configure: dist.missing-tools   := True
configure: build.configure-args := ['--enable-sccache', '--disable-manage-submodu ...
configure: writing `config.toml` in current directory
configure: 
configure: run `python /checkout/x.py --help`
configure: 
---
 Documenting core v0.0.0 (/checkout/library/core)
error: unresolved link to `tag`
   --> library/core/src/any.rs:814:7
    |
814 | /// [`tag`] module, although crates may also implement their own tags for more
    |       ^^^ no item named `tag` in scope
    |
    = note: `-D rustdoc::broken-intra-doc-links` implied by `-D warnings`
    = help: to escape `[` and `]` characters, add '\' before them like `\[` or `\]`
error: could not document `core`

Caused by:
Caused by:
  process didn't exit successfully: `/checkout/obj/build/bootstrap/debug/rustdoc --edition=2018 --crate-type lib --crate-name core library/core/src/lib.rs --target x86_64-unknown-linux-gnu -o /checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/doc --error-format=json --json=diagnostic-rendered-ansi --markdown-css rust.css --markdown-no-toc -Z unstable-options --resource-suffix 1.59.0 --index-page /checkout/src/doc/index.md -C metadata=4599449636879869 -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/release/deps --cfg=bootstrap -Zsymbol-mangling-version=legacy -Dwarnings '-Wrustdoc::invalid_codeblock_attributes' --crate-version '1.59.0-nightly
  (98b68f7f0
  2021-12-16)' '-Zcrate-attr=doc(html_root_url="https://doc.rust-lang.org/nightly/")'` (exit status: 1)


command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustdoc" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "16" "--release" "--locked" "--color" "always" "--features" "panic-unwind backtrace compiler-builtins-c" "--manifest-path" "/checkout/library/test/Cargo.toml" "-p" "core" "-Zskip-rustdoc-fingerprint" "--" "--markdown-css" "rust.css" "--markdown-no-toc" "-Z" "unstable-options" "--resource-suffix" "1.59.0" "--index-page" "/checkout/src/doc/index.md"


Build completed unsuccessfully in 0:00:07
