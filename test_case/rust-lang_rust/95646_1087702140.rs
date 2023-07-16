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
error: unresolved link to `std::env::var`
   --> library/core/src/macros/mod.rs:913:11
    |
913 |     /// [`std::env::var`] instead if you want to read the value at runtime.
    |           ^^^^^^^^^^^^^ no item named `std` in scope
    |
    = note: `-D rustdoc::broken-intra-doc-links` implied by `-D warnings`
error: unresolved link to `std::env::var`
   --> library/core/src/macros/mod.rs:955:11
    |
    |
955 |     /// [`std::env::var`] instead if you want to read the value at runtime.
    |           ^^^^^^^^^^^^^ no item named `std` in scope
error: could not document `core`

Caused by:
Caused by:
  process didn't exit successfully: `/checkout/obj/build/bootstrap/debug/rustdoc --edition=2021 --crate-type lib --crate-name core library/core/src/lib.rs --target x86_64-unknown-linux-gnu -o /checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/doc --error-format=json --json=diagnostic-rendered-ansi,artifacts,future-incompat --markdown-css rust.css --markdown-no-toc -Z unstable-options --resource-suffix 1.62.0 --index-page /checkout/src/doc/index.md -C metadata=4599449636879869 -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/release/deps --cfg=bootstrap -Csymbol-mangling-version=legacy -Zunstable-options -Dwarnings '-Wrustdoc::invalid_codeblock_attributes' --crate-version '1.62.0-nightly
  (f9c6acb17
  2022-04-04)' '-Zcrate-attr=doc(html_root_url="https://doc.rust-lang.org/nightly/")'` (exit status: 1)
