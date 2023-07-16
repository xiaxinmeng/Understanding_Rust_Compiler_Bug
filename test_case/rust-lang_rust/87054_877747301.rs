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
 Documenting core v0.0.0 (/checkout/library/core)
error: unresolved link to `try_fold`
    --> library/core/src/iter/traits/iterator.rs:2227:10
     |
2227 |     /// [`try_fold()`] with the first element of the iterator as the initial
     |          ^^^^^^^^^^^^ no item named `try_fold` in scope
     |
     = note: `-D rustdoc::broken-intra-doc-links` implied by `-D warnings`
     = help: to escape `[` and `]` characters, add '\' before them like `\[` or `\]`
error: unresolved link to `try_fold`
    --> library/core/src/iter/traits/iterator.rs:2227:10
     |
     |
2227 |     /// [`try_fold()`] with the first element of the iterator as the initial
     |          ^^^^^^^^^^^^ no item named `try_fold` in scope
     |
     = help: to escape `[` and `]` characters, add '\' before them like `\[` or `\]`
error: aborting due to 2 previous errors

error: could not document `core`


Caused by:
  process didn't exit successfully: `/checkout/obj/build/bootstrap/debug/rustdoc --edition=2018 --crate-type lib --crate-name core library/core/src/lib.rs --target x86_64-unknown-linux-gnu -o /checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/doc --error-format=json --json=diagnostic-rendered-ansi --markdown-css rust.css --markdown-no-toc -Z unstable-options --resource-suffix 1.55.0 --index-page /checkout/src/doc/index.md -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/release/deps --cfg=bootstrap -Dwarnings '-Wrustdoc::invalid_codeblock_attributes' --crate-version '1.55.0-nightly
  (ce1d320eb
  2021-07-11)' '-Zcrate-attr=doc(html_root_url="https://doc.rust-lang.org/nightly/")'` (exit status: 1)


command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustdoc" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "16" "--release" "--locked" "--color" "always" "--features" "panic-unwind backtrace compiler-builtins-c" "--manifest-path" "/checkout/library/test/Cargo.toml" "-p" "core" "-Zskip-rustdoc-fingerprint" "--" "--markdown-css" "rust.css" "--markdown-no-toc" "-Z" "unstable-options" "--resource-suffix" "1.55.0" "--index-page" "/checkout/src/doc/index.md"


Build completed unsuccessfully in 0:00:05
