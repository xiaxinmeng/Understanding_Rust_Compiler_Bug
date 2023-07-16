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
error: unresolved link to `std::ops::ControlFlow`
    --> library/core/src/iter/traits/iterator.rs:2003:55
     |
2003 |     /// While you cannot `break` from a closure, the [`std::ops::ControlFlow`]
     |                                                       ^^^^^^^^^^^^^^^^^^^^^^^ no item named `std` in scope
     |
     = note: `-D rustdoc::broken-intra-doc-links` implied by `-D warnings`
error: unresolved link to `std::ops::ControlFlow`
    --> library/core/src/iter/traits/iterator.rs:2070:14
     |
     |
2070 |     /// The [`std::ops::ControlFlow`] type can be used with this method for the
     |              ^^^^^^^^^^^^^^^^^^^^^^^ no item named `std` in scope
error: unresolved link to `std::ops::ControlFlow`
    --> library/core/src/iter/traits/iterator.rs:2003:55
     |
     |
2003 |     /// While you cannot `break` from a closure, the [`std::ops::ControlFlow`]
     |                                                       ^^^^^^^^^^^^^^^^^^^^^^^ no item named `std` in scope
error: aborting due to 3 previous errors

error: could not document `core`


Caused by:
  process didn't exit successfully: `/checkout/obj/build/bootstrap/debug/rustdoc --edition=2018 --crate-type lib --crate-name core library/core/src/lib.rs --target x86_64-unknown-linux-gnu -o /checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/doc --error-format=json --json=diagnostic-rendered-ansi --markdown-css rust.css --markdown-no-toc -Z unstable-options --resource-suffix 1.54.0 --index-page /checkout/src/doc/index.md -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/release/deps --cfg=bootstrap -Dwarnings '-Wrustdoc::invalid_codeblock_attributes' --crate-version '1.54.0-nightly
  (e702faa5f
  2021-05-23)'` (exit code: 1)


command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustdoc" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "16" "--release" "--locked" "--color" "always" "--features" "panic-unwind backtrace compiler-builtins-c" "--manifest-path" "/checkout/library/test/Cargo.toml" "-p" "core" "--" "--markdown-css" "rust.css" "--markdown-no-toc" "-Z" "unstable-options" "--resource-suffix" "1.54.0" "--index-page" "/checkout/src/doc/index.md"


failed to run: /checkout/obj/build/bootstrap/debug/bootstrap doc --stage 0 library/test
Build completed unsuccessfully in 0:00:07
