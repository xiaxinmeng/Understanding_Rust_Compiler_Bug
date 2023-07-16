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
    Checking core v0.0.0 (/checkout/library/core)
    Checking rustc-std-workspace-core v1.99.0 (/checkout/library/rustc-std-workspace-core)
    Checking compiler_builtins v0.1.39
 Documenting alloc v0.0.0 (/checkout/library/alloc)
error: unresolved link to `super::rc::Rc::make_mut`
    --> library/alloc/src/sync/arc.rs:1266:53
     |
1266 |     /// Note that this differs from the behavior of [`Rc::make_mut`] which disassociates
     |                                                     ^^^^^^^^^^^^^^^^ no item named `rc` in module `sync`
     |
     = note: `-D broken-intra-doc-links` implied by `-D warnings`
error: aborting due to previous error

error: could not document `alloc`


Caused by:
  process didn't exit successfully: `/checkout/obj/build/bootstrap/debug/rustdoc --edition=2018 --crate-type lib --crate-name alloc library/alloc/src/lib.rs --target x86_64-unknown-linux-gnu -o /checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/doc --cfg 'feature="compiler-builtins-c"' --error-format=json --json=diagnostic-rendered-ansi --markdown-css rust.css --markdown-no-toc -Z unstable-options --resource-suffix 1.51.0 --index-page /checkout/src/doc/index.md -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/release/deps --extern compiler_builtins=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps/libcompiler_builtins-cb5dc68e64b15c92.rmeta --extern core=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps/libcore-4e0337fd1372e206.rmeta --cfg=bootstrap -Dwarnings -Winvalid_codeblock_attributes --crate-version '1.51.0-nightly
  (79ef66ac2
  2021-01-22)'` (exit code: 1)


command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustdoc" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "16" "--release" "--locked" "--color" "always" "--features" "panic-unwind backtrace compiler-builtins-c" "--manifest-path" "/checkout/library/test/Cargo.toml" "-p" "alloc" "--" "--markdown-css" "rust.css" "--markdown-no-toc" "-Z" "unstable-options" "--resource-suffix" "1.51.0" "--index-page" "/checkout/src/doc/index.md"


failed to run: /checkout/obj/build/bootstrap/debug/bootstrap doc --stage 0 library/std
Build completed unsuccessfully in 0:00:34
