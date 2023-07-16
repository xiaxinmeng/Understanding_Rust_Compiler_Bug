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
 Documenting alloc v0.0.0 (/checkout/library/alloc)
error: unresolved link to `crate::num`
  --> library/alloc/src/option.rs:91:24
   |
91 | //! [`num::NonZero*`]: crate::num
   |                        ^^^^^^^^^^ no item named `num` in module `alloc`
   |
   = note: `-D rustdoc::broken-intra-doc-links` implied by `-D warnings`
error: unresolved link to `crate::ptr::NonNull`
  --> library/alloc/src/option.rs:92:26
   |
   |
92 | //! [`ptr::NonNull<U>`]: crate::ptr::NonNull
   |                          ^^^^^^^^^^^^^^^^^^^ no item named `ptr` in module `alloc`
error: unresolved link to `mem::transmute`
  --> library/alloc/src/option.rs:97:7
   |
   |
97 | //! [`mem::transmute`] from all valid values of `T` to `Option<T>` and
   |       ^^^^^^^^^^^^^^ no item named `mem` in scope
error: unresolved link to `Deref::Target`
   --> library/alloc/src/option.rs:129:15
    |
    |
129 | //! [Target]: Deref::Target "ops::Deref::Target"

error: unresolved link to `Pin`
   --> library/alloc/src/option.rs:122:43
    |
    |
122 | //! * [`as_pin_ref`] converts from <code>[Pin]<[&][][Option]\<T>></code> to
    |                                           ^^^ no item named `Pin` in scope
    |
    = help: to escape `[` and `]` characters, add '\' before them like `\[` or `\]`
error: unresolved link to `Pin`
   --> library/alloc/src/option.rs:123:23
    |
    |
123 | //!   <code>[Option]<[Pin]<[&]T>></code>
    |                       ^^^ no item named `Pin` in scope
    |
    = help: to escape `[` and `]` characters, add '\' before them like `\[` or `\]`
error: unresolved link to `Pin`
   --> library/alloc/src/option.rs:124:43
    |
    |
124 | //! * [`as_pin_mut`] converts from <code>[Pin]<[&mut] [Option]\<T>></code> to
    |                                           ^^^ no item named `Pin` in scope
    |
    = help: to escape `[` and `]` characters, add '\' before them like `\[` or `\]`
error: unresolved link to `Pin`
   --> library/alloc/src/option.rs:125:23
    |
    |
125 | //!   <code>[Option]<[Pin]<[&mut] T>></code>
    |                       ^^^ no item named `Pin` in scope
    |
    = help: to escape `[` and `]` characters, add '\' before them like `\[` or `\]`
error: unresolved link to `crate::iter::once`
   --> library/alloc/src/option.rs:318:18
    |
    |
318 | //! [`once(v)`]: crate::iter::once
    |                  ^^^^^^^^^^^^^^^^^ no item named `iter` in module `alloc`
error: unresolved link to `crate::iter::empty`
   --> library/alloc/src/option.rs:317:18
    |
    |
317 | //! [`empty()`]: crate::iter::empty
    |                  ^^^^^^^^^^^^^^^^^^ no item named `iter` in module `alloc`
error: could not document `alloc`

Caused by:
Caused by:
  process didn't exit successfully: `/checkout/obj/build/bootstrap/debug/rustdoc --edition=2021 --crate-type lib --crate-name alloc library/alloc/src/lib.rs --target x86_64-unknown-linux-gnu -o /checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/doc --cfg 'feature="compiler-builtins-c"' --error-format=json --json=diagnostic-rendered-ansi,artifacts,future-incompat --markdown-css rust.css --markdown-no-toc -Z unstable-options --resource-suffix 1.63.0 --index-page /checkout/src/doc/index.md -C metadata=67f24b18eb44654e -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/release/deps --extern compiler_builtins=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps/libcompiler_builtins-452860faad4d9c04.rmeta --extern core=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps/libcore-07ea62845071a1d9.rmeta --cfg=bootstrap -Csymbol-mangling-version=legacy -Zunstable-options -Dwarnings '-Wrustdoc::invalid_codeblock_attributes' --crate-version '1.63.0-nightly
  (3e8836a2f
  2022-06-13)' '-Zcrate-attr=doc(html_root_url="https://doc.rust-lang.org/nightly/")'` (exit status: 1)
