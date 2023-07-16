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
Building stage0 std artifacts (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
    Finished release [optimized] target(s) in 0.17s
Copying stage0 std from stage0 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu / x86_64-unknown-linux-gnu)
 Documenting core v0.0.0 (/checkout/library/core)
error: unresolved link to `pointer::zst_exists`
   |
   |
96 | //! [`zst_exists`]: pointer::zst_exists
   |                     ^^^^^^^^^^^^^^^^^^^ the builtin type `pointer` has no associated item named `zst_exists`
   |
   = note: `-D rustdoc::broken-intra-doc-links` implied by `-D warnings`
error: unresolved link to `addr`
  --> library/core/src/ptr/const_ptr.rs:70:15
   |
   |
70 |     /// see [`addr`][] and [`with_addr`][] for the responsible way to do that.
   |               ^^^^ no item named `addr` in scope
   |
   = help: to escape `[` and `]` characters, add '\' before them like `\[` or `\]`

error: unresolved link to `with_addr`
   |
   |
70 |     /// see [`addr`][] and [`with_addr`][] for the responsible way to do that.
   |                              ^^^^^^^^^ no item named `with_addr` in scope
   |
   = help: to escape `[` and `]` characters, add '\' before them like `\[` or `\]`

error: unresolved link to `with_addr`
    |
    |
112 |     /// this may remove some important metadata. See [`with_addr`][] for
    |                                                        ^^^^^^^^^ no item named `with_addr` in scope
    |
    = help: to escape `[` and `]` characters, add '\' before them like `\[` or `\]`

error: unresolved link to `ptr::fake_alloc`
    |
    |
125 |     /// See also: [`ptr::fake_alloc`][] and [`ptr::zst_exists`][].
    |                     ^^^^^^^^^^^^^^^ no item named `ptr` in scope

error: unresolved link to `ptr::zst_exists`
    |
    |
125 |     /// See also: [`ptr::fake_alloc`][] and [`ptr::zst_exists`][].
    |                                               ^^^^^^^^^^^^^^^ no item named `ptr` in scope
error: unresolved link to `addr`
  --> library/core/src/ptr/mut_ptr.rs:73:15
   |
   |
73 |     /// see [`addr`][] and [`with_addr`][] for the responsible way to do that.
   |               ^^^^ no item named `addr` in scope
   |
   = help: to escape `[` and `]` characters, add '\' before them like `\[` or `\]`

error: unresolved link to `with_addr`
   |
   |
73 |     /// see [`addr`][] and [`with_addr`][] for the responsible way to do that.
   |                              ^^^^^^^^^ no item named `with_addr` in scope
   |
   = help: to escape `[` and `]` characters, add '\' before them like `\[` or `\]`

error: unresolved link to `with_addr`
    |
    |
115 |     /// this may remove some important metadata. See [`with_addr`][] for
    |                                                        ^^^^^^^^^ no item named `with_addr` in scope
    |
    = help: to escape `[` and `]` characters, add '\' before them like `\[` or `\]`

error: unresolved link to `ptr::fake_alloc`
    |
    |
128 |     /// See also: [`ptr::fake_alloc`][] and [`ptr::zst_exists`][].
    |                     ^^^^^^^^^^^^^^^ no item named `ptr` in scope

error: unresolved link to `ptr::zst_exists`
    |
    |
128 |     /// See also: [`ptr::fake_alloc`][] and [`ptr::zst_exists`][].
    |                                               ^^^^^^^^^^^^^^^ no item named `ptr` in scope

error: unresolved link to `with_addr`
    |
    |
340 | /// See [`with_addr`] for more details.
    |           ^^^^^^^^^ no item named `with_addr` in scope
    |
    = help: to escape `[` and `]` characters, add '\' before them like `\[` or `\]`
error: could not document `core`

Caused by:
Caused by:
  process didn't exit successfully: `/checkout/obj/build/bootstrap/debug/rustdoc --edition=2021 --crate-type lib --crate-name core library/core/src/lib.rs --target x86_64-unknown-linux-gnu -o /checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/doc --error-format=json --json=diagnostic-rendered-ansi,artifacts,future-incompat --markdown-css rust.css --markdown-no-toc -Z unstable-options --resource-suffix 1.61.0 --index-page /checkout/src/doc/index.md -C metadata=4599449636879869 -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/release/deps --cfg=bootstrap -Csymbol-mangling-version=legacy -Zunstable-options -Dwarnings '-Wrustdoc::invalid_codeblock_attributes' --crate-version '1.61.0-nightly
  (d10e87a9c
  2022-03-23)' '-Zcrate-attr=doc(html_root_url="https://doc.rust-lang.org/nightly/")'` (exit status: 1)
