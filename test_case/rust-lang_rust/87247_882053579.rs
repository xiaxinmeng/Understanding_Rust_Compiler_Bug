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
 Documenting test v0.0.0 (/checkout/library/test)
error: unresolved link to `ansi`
 --> library/test/src/term.rs:4:15
  |
4 | //! Terminal][ansi] to provide color printing, among other things. There are two
  |               ^^^^ no item named `ansi` in scope
  |
  = note: `-D rustdoc::broken-intra-doc-links` implied by `-D warnings`
  = help: to escape `[` and `]` characters, add '\' before them like `\[` or `\]`
error: unresolved link to `ti`
 --> library/test/src/term.rs:6:18
  |
  |
6 | //! a [terminfo][ti] database, and `WinConsole`, which uses the [Win32 Console
  |                  ^^ no item named `ti` in scope
  |
  = help: to escape `[` and `]` characters, add '\' before them like `\[` or `\]`
error: unresolved link to `win`
 --> library/test/src/term.rs:7:10
  |
  |
7 | //! API][win].
  |          ^^^ no item named `win` in scope
  |
  = help: to escape `[` and `]` characters, add '\' before them like `\[` or `\]`
error: aborting due to 3 previous errors

error: could not document `test`


Caused by:
  process didn't exit successfully: `/checkout/obj/build/bootstrap/debug/rustdoc --edition=2018 --crate-type dylib --crate-type rlib --crate-name test library/test/src/lib.rs --target x86_64-unknown-linux-gnu -o /checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/doc --cfg 'feature="backtrace"' --cfg 'feature="compiler-builtins-c"' --cfg 'feature="default"' --cfg 'feature="panic-unwind"' --cfg 'feature="std_detect_dlsym_getauxval"' --cfg 'feature="std_detect_file_io"' --error-format=json --json=diagnostic-rendered-ansi --markdown-css rust.css --markdown-no-toc -Z unstable-options --resource-suffix 1.55.0 --index-page /checkout/src/doc/index.md -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/release/deps --extern cfg_if=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps/libcfg_if-cb21ef14fc5b2a44.rmeta --extern core=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps/libcore-730f5e0d8d7f0fbb.rmeta --extern getopts=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps/libgetopts-0829fc985d39cf14.rmeta --extern libc=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps/liblibc-723b91699d323eb0.rmeta --extern panic_abort=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps/libpanic_abort-137ee313240d7d56.rmeta --extern panic_unwind=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps/libpanic_unwind-d369bb2a3c548a06.rmeta --extern proc_macro=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps/libproc_macro-c9735c862091044c.rmeta --extern std=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps/libstd-5e377cbd22aff3ce.rmeta --cfg=bootstrap -Dwarnings '-Wrustdoc::invalid_codeblock_attributes' --crate-version '1.55.0-nightly
  (bd4cbec99
  2021-07-18)' '-Zcrate-attr=doc(html_root_url="https://doc.rust-lang.org/nightly/")'` (exit status: 1)


command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustdoc" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "16" "--release" "--locked" "--color" "always" "--features" "panic-unwind backtrace compiler-builtins-c" "--manifest-path" "/checkout/library/test/Cargo.toml" "-p" "test" "-Zskip-rustdoc-fingerprint" "--" "--markdown-css" "rust.css" "--markdown-no-toc" "-Z" "unstable-options" "--resource-suffix" "1.55.0" "--index-page" "/checkout/src/doc/index.md"


Build completed unsuccessfully in 0:00:36
