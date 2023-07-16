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
 Documenting std v0.0.0 (/checkout/library/std)
error: unresolved link to `catch_unwind`
  --> library/std/src/error.rs:22:9
   |
22 | //! * [`catch_unwind`] and [`resume_unwind`] (Discarding, Propagating)
   |         ^^^^^^^^^^^^ no item named `catch_unwind` in scope
   |
   = note: `-D rustdoc::broken-intra-doc-links` implied by `-D warnings`
   = help: to escape `[` and `]` characters, add '\' before them like `\[` or `\]`
error: unresolved link to `resume_unwind`
  --> library/std/src/error.rs:22:30
   |
   |
22 | //! * [`catch_unwind`] and [`resume_unwind`] (Discarding, Propagating)
   |
   |
   = help: to escape `[` and `]` characters, add '\' before them like `\[` or `\]`

error: unresolved link to `downcast`
   |
   |
30 | //! * `match` and [`downcast`] (Reacting)
   |                     ^^^^^^^^ no item named `downcast` in scope
   |
   = help: to escape `[` and `]` characters, add '\' before them like `\[` or `\]`
error: unresolved link to `Try`
  --> library/std/src/error.rs:32:30
   |
   |
32 | //! * The partially stable [`Try`] traits (Propagating, Constructing)
   |                              ^^^ no item named `Try` in scope
   |
   = help: to escape `[` and `]` characters, add '\' before them like `\[` or `\]`
error: unresolved link to `Termination`
  --> library/std/src/error.rs:33:9
   |
   |
33 | //! * [`Termination`] (Reporting)
   |         ^^^^^^^^^^^ no item named `Termination` in scope
   |
   = help: to escape `[` and `]` characters, add '\' before them like `\[` or `\]`
error: unresolved link to `catch_unwind`
   --> library/std/src/error.rs:131:7
    |
    |
131 | //! [`catch_unwind`]: crate::panic::catch_unwind
    |       ^^^^^^^^^^^^ no item named `catch_unwind` in scope
    |
    = help: to escape `[` and `]` characters, add '\' before them like `\[` or `\]`
error: unresolved link to `resume_unwind`
   --> library/std/src/error.rs:132:7
    |
    |
132 | //! [`resume_unwind`]: crate::panic::resume_unwind
    |
    |
    = help: to escape `[` and `]` characters, add '\' before them like `\[` or `\]`
error: unresolved link to `Termination`
   --> library/std/src/error.rs:133:7
    |
    |
133 | //! [`Termination`]: crate::process::Termination
    |       ^^^^^^^^^^^ no item named `Termination` in scope
    |
    = help: to escape `[` and `]` characters, add '\' before them like `\[` or `\]`
error: unresolved link to `Try`
   --> library/std/src/error.rs:134:7
    |
    |
134 | //! [`Try`]: crate::ops::Try
    |       ^^^ no item named `Try` in scope
    |
    = help: to escape `[` and `]` characters, add '\' before them like `\[` or `\]`

error: unresolved link to `downcast`
    |
    |
135 | //! [`downcast`]: crate::error::Error
    |       ^^^^^^^^ no item named `downcast` in scope
    |
    = help: to escape `[` and `]` characters, add '\' before them like `\[` or `\]`

error: this URL is not a hyperlink
    |
    |
130 | //! [`#[panic_handler]`]: https://doc.rust-lang.org/nomicon/panic-handler.html
    |                           ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: use an automatic link instead: `<https://doc.rust-lang.org/nomicon/panic-handler.html>`
    |
    = note: `-D rustdoc::bare-urls` implied by `-D warnings`
    = note: bare URLs are not automatically turned into clickable links
error: could not document `std`

Caused by:
Caused by:
  process didn't exit successfully: `/checkout/obj/build/bootstrap/debug/rustdoc --edition=2021 --crate-type dylib --crate-type rlib --crate-name std library/std/src/lib.rs --target x86_64-unknown-linux-gnu -o /checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/doc --cfg 'feature="addr2line"' --cfg 'feature="backtrace"' --cfg 'feature="compiler-builtins-c"' --cfg 'feature="gimli-symbolize"' --cfg 'feature="miniz_oxide"' --cfg 'feature="object"' --cfg 'feature="panic_unwind"' --cfg 'feature="std_detect_dlsym_getauxval"' --cfg 'feature="std_detect_file_io"' --error-format=json --json=diagnostic-rendered-ansi,artifacts,future-incompat --markdown-css rust.css --markdown-no-toc -Z unstable-options --resource-suffix 1.63.0 --index-page /checkout/src/doc/index.md -C metadata=9277900c0e11838b -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/release/deps --extern addr2line=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps/libaddr2line-f10831a42b1025a6.rmeta --extern alloc=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps/liballoc-9cd0cf8c38b27881.rmeta --extern cfg_if=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps/libcfg_if-93b9fb0724cf2f7f.rmeta --extern compiler_builtins=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps/libcompiler_builtins-d2c2e17e5f70a681.rmeta --extern core=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps/libcore-07ea62845071a1d9.rmeta --extern hashbrown=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps/libhashbrown-d1fb8db9a8c70750.rmeta --extern libc=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps/liblibc-483b19eaeb3390b2.rmeta --extern miniz_oxide=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps/libminiz_oxide-dc7906d34abe6f74.rmeta --extern object=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps/libobject-02d4d9d31f0023f8.rmeta --extern panic_abort=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps/libpanic_abort-1a5c6eef5dd37639.rmeta --extern panic_unwind=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps/libpanic_unwind-4f9af54b22e6963c.rmeta --extern rustc_demangle=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps/librustc_demangle-7a495c96c19fb96b.rmeta --extern std_detect=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps/libstd_detect-3c20b325373716de.rmeta --extern unwind=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps/libunwind-b3b532784766a653.rmeta --cfg=bootstrap -Csymbol-mangling-version=legacy -Zunstable-options -Dwarnings '-Wrustdoc::invalid_codeblock_attributes' --crate-version '1.63.0-nightly
  (33b08273a
  2022-05-24)' '-Zcrate-attr=doc(html_root_url="https://doc.rust-lang.org/nightly/")' --cfg backtrace_in_libstd` (exit status: 1)
