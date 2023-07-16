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
error: unresolved link to `CString`
  --> library/core/src/ffi/c_str.rs:30:21
   |
30 | /// into an owned [`CString`].
   |                     ^^^^^^^ no item named `CString` in scope
   |
   = note: `-D rustdoc::broken-intra-doc-links` implied by `-D warnings`
   = help: to escape `[` and `]` characters, add '\' before them like `\[` or `\]`
error: unresolved link to `CString`
  --> library/core/src/ffi/c_str.rs:32:21
   |
   |
32 | /// `&CStr` is to [`CString`] as <code>&[str]</code> is to [`String`]: the former
   |                     ^^^^^^^ no item named `CString` in scope
   |
   = help: to escape `[` and `]` characters, add '\' before them like `\[` or `\]`
error: unresolved link to `String`
  --> library/core/src/ffi/c_str.rs:32:62
   |
   |
32 | /// `&CStr` is to [`CString`] as <code>&[str]</code> is to [`String`]: the former
   |                                                              ^^^^^^ no item named `String` in scope
   |
   = help: to escape `[` and `]` characters, add '\' before them like `\[` or `\]`
error: unresolved link to `String`
  --> library/core/src/ffi/c_str.rs:73:49
   |
   |
73 | /// Converting a foreign C string into a Rust [`String`]:
   |                                                 ^^^^^^ no item named `String` in scope
   |
   = help: to escape `[` and `]` characters, add '\' before them like `\[` or `\]`
error: unresolved link to `CString`
   --> library/core/src/ffi/c_str.rs:349:40
    |
    |
349 |     /// lifetime information and the [`CString`] is deallocated immediately after
    |                                        ^^^^^^^ no item named `CString` in scope
    |
    = help: to escape `[` and `]` characters, add '\' before them like `\[` or `\]`
error: unresolved link to `CString`
   --> library/core/src/ffi/c_str.rs:366:41
    |
    |
366 |     /// This way, the lifetime of the [`CString`] in `hello` encompasses
    |                                         ^^^^^^^ no item named `CString` in scope
    |
    = help: to escape `[` and `]` characters, add '\' before them like `\[` or `\]`
error: could not document `core`

Caused by:
Caused by:
  process didn't exit successfully: `/checkout/obj/build/bootstrap/debug/rustdoc --edition=2021 --crate-type lib --crate-name core library/core/src/lib.rs --target x86_64-unknown-linux-gnu -o /checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/doc --error-format=json --json=diagnostic-rendered-ansi,future-incompat --markdown-css rust.css --markdown-no-toc -Z unstable-options --resource-suffix 1.60.0 --index-page /checkout/src/doc/index.md -C metadata=4599449636879869 -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/release/deps --cfg=bootstrap -Csymbol-mangling-version=legacy -Zunstable-options -Dwarnings '-Wrustdoc::invalid_codeblock_attributes' --crate-version '1.60.0-nightly
  (9fd3bfba1
  2022-02-18)' '-Zcrate-attr=doc(html_root_url="https://doc.rust-lang.org/nightly/")'` (exit status: 1)
