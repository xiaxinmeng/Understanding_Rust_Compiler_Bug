plain
Set({doc::src/doc}) not skipped for "bootstrap::doc::Standalone" -- not in [src/tools/tidy]
Set({doc::library/alloc, doc::library/core, doc::library/panic_abort, doc::library/panic_unwind, doc::library/proc_macro, doc::library/std, doc::library/test, doc::library/unwind}) not skipped for "bootstrap::doc::Std" -- not in [src/tools/tidy]
Documenting stage2 std (x86_64-unknown-linux-gnu)
 Documenting core v0.0.0 (/checkout/library/core)
error: unresolved link to `sort_unstable_by`
     |
     |
3997 |     /// This uses the same sorting algorithm as [`sort_unstable_by`].
     |                                                   ^^^^^^^^^^^^^^^^ no item named `sort_unstable_by` in scope
     |
     = note: `-D rustdoc::broken-intra-doc-links` implied by `-D warnings`
     = help: to escape `[` and `]` characters, add '\' before them like `\[` or `\]`

error: unresolved link to `sort_unstable_by`
     |
     |
4028 |     /// This uses the same sorting algorithm as [`sort_unstable_by`].
     |                                                   ^^^^^^^^^^^^^^^^ no item named `sort_unstable_by` in scope
     |
     = help: to escape `[` and `]` characters, add '\' before them like `\[` or `\]`
error: could not document `core`

Caused by:
Caused by:
  process didn't exit successfully: `/checkout/obj/build/bootstrap/debug/rustdoc --edition=2021 --crate-type lib --crate-name core library/core/src/lib.rs --target x86_64-unknown-linux-gnu -o /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-unknown-linux-gnu/doc --error-format=json --json=diagnostic-rendered-ansi --markdown-css rust.css --markdown-no-toc -Z unstable-options --resource-suffix 1.60.0 --index-page /checkout/src/doc/index.md -C metadata=f4ae526a78b232e4 -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-unknown-linux-gnu/release/deps -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/release/deps -Csymbol-mangling-version=legacy -Zunstable-options -Dwarnings '-Wrustdoc::invalid_codeblock_attributes' --crate-version '1.60.0-nightly
  (5dbd75093
  2022-01-28)' '-Zcrate-attr=doc(html_root_url="https://doc.rust-lang.org/nightly/")'` (exit status: 1)


command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustdoc" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "16" "--release" "--locked" "--color" "always" "--features" "panic-unwind backtrace compiler-builtins-c" "--manifest-path" "/checkout/library/test/Cargo.toml" "-p" "core" "-Zskip-rustdoc-fingerprint" "--" "--markdown-css" "rust.css" "--markdown-no-toc" "-Z" "unstable-options" "--resource-suffix" "1.60.0" "--index-page" "/checkout/src/doc/index.md"


Build completed unsuccessfully in 0:26:14
