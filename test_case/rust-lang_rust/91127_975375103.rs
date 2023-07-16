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
error: unresolved link to `Self::from_bits`
  --> library/core/src/ptr/const_ptr.rs:54:33
   |
54 |     /// The inverse method is [`Self::from_bits`].
   |                                 ^^^^^^^^^^^^^^^ no item named `Self` in scope
   |
   = note: `-D rustdoc::broken-intra-doc-links` implied by `-D warnings`
error: unresolved link to `Self::to_bits`
  --> library/core/src/ptr/const_ptr.rs:81:33
   |
   |
81 |     /// The inverse method is [`Self::to_bits`].
   |                                 ^^^^^^^^^^^^^ no item named `Self` in scope
error: unresolved link to `Self::from_bits`
  --> library/core/src/ptr/mut_ptr.rs:53:33
   |
   |
53 |     /// The inverse method is [`Self::from_bits`].
   |                                 ^^^^^^^^^^^^^^^ no item named `Self` in scope
error: unresolved link to `Self::to_bits`
  --> library/core/src/ptr/mut_ptr.rs:81:33
   |
   |
81 |     /// The inverse method is [`Self::to_bits`].
   |                                 ^^^^^^^^^^^^^ no item named `Self` in scope
error: could not document `core`

Caused by:
Caused by:
  process didn't exit successfully: `/checkout/obj/build/bootstrap/debug/rustdoc --edition=2018 --crate-type lib --crate-name core library/core/src/lib.rs --target x86_64-unknown-linux-gnu -o /checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/doc --error-format=json --json=diagnostic-rendered-ansi --markdown-css rust.css --markdown-no-toc -Z unstable-options --resource-suffix 1.58.0 --index-page /checkout/src/doc/index.md -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/release/deps --cfg=bootstrap -Zsymbol-mangling-version=legacy -Dwarnings '-Wrustdoc::invalid_codeblock_attributes' --crate-version '1.58.0-nightly
  (d1c516a06
  2021-11-22)' '-Zcrate-attr=doc(html_root_url="https://doc.rust-lang.org/nightly/")'` (exit status: 1)


command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustdoc" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "16" "--release" "--locked" "--color" "always" "--features" "panic-unwind backtrace compiler-builtins-c" "--manifest-path" "/checkout/library/test/Cargo.toml" "-p" "core" "-Zskip-rustdoc-fingerprint" "--" "--markdown-css" "rust.css" "--markdown-no-toc" "-Z" "unstable-options" "--resource-suffix" "1.58.0" "--index-page" "/checkout/src/doc/index.md"


Build completed unsuccessfully in 0:00:07
