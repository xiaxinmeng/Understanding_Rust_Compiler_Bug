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
 Documenting alloc v0.0.0 (/checkout/library/alloc)
error: unresolved link to `push`
    --> library/alloc/src/vec/mod.rs:1680:18
     |
1680 |     /// Unlike [`push`] method will not reallocate when there's insufficient capacity.
     |                  ^^^^ no item named `push` in scope
     |
     = note: `-D rustdoc::broken-intra-doc-links` implied by `-D warnings`
     = help: to escape `[` and `]` characters, add '\' before them like `\[` or `\]`
error: unresolved link to `reserve`
    --> library/alloc/src/vec/mod.rs:1681:33
     |
     |
1681 |     /// The caller should use [`reserve`] or [`try_reserve`] to ensure that there is enough capacity.
     |                                 ^^^^^^^ no item named `reserve` in scope
     |
     = help: to escape `[` and `]` characters, add '\' before them like `\[` or `\]`
error: unresolved link to `try_reserve`
    --> library/alloc/src/vec/mod.rs:1681:48
     |
     |
1681 |     /// The caller should use [`reserve`] or [`try_reserve`] to ensure that there is enough capacity.
     |                                                ^^^^^^^^^^^ no item named `try_reserve` in scope
     |
     = help: to escape `[` and `]` characters, add '\' before them like `\[` or `\]`
error: could not document `alloc`

Caused by:
Caused by:
  process didn't exit successfully: `/checkout/obj/build/bootstrap/debug/rustdoc --edition=2018 --crate-type lib --crate-name alloc library/alloc/src/lib.rs --target x86_64-unknown-linux-gnu -o /checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/doc --cfg 'feature="compiler-builtins-c"' --error-format=json --json=diagnostic-rendered-ansi --markdown-css rust.css --markdown-no-toc -Z unstable-options --resource-suffix 1.57.0 --index-page /checkout/src/doc/index.md -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/release/deps --extern compiler_builtins=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps/libcompiler_builtins-0ed769fd3e253445.rmeta --extern core=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps/libcore-df348a3b7744a664.rmeta --cfg=bootstrap -Dwarnings '-Wrustdoc::invalid_codeblock_attributes' --crate-version '1.57.0-nightly
  (a56988b9c
  2021-09-20)' '-Zcrate-attr=doc(html_root_url="https://doc.rust-lang.org/nightly/")'` (exit status: 1)


command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustdoc" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "16" "--release" "--locked" "--color" "always" "--features" "panic-unwind backtrace compiler-builtins-c" "--manifest-path" "/checkout/library/test/Cargo.toml" "-p" "alloc" "-Zskip-rustdoc-fingerprint" "--" "--markdown-css" "rust.css" "--markdown-no-toc" "-Z" "unstable-options" "--resource-suffix" "1.57.0" "--index-page" "/checkout/src/doc/index.md"


Build completed unsuccessfully in 0:00:26
