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
 Documenting std v0.0.0 (/checkout/library/std)
error: unresolved link to `read`
   --> library/std/src/io/mod.rs:771:36
    |
771 |     /// This is equivalent to the [`read`] method, except that it is passed a [`ReadBuf`] rather than `[u8]` to allow use
    |                                    ^^^^^^ no item named `read` in scope
    |
    = note: `-D broken-intra-doc-links` implied by `-D warnings`
    = help: to escape `[` and `]` characters, add '\' before them like `\[` or `\]`
error: unresolved link to `read_exact`
   --> library/std/src/io/mod.rs:784:36
    |
    |
784 |     /// This is equivalent to the [`read_exact`] method, except that it is passed a [`ReadBuf`] rather than `[u8]` to
    |                                    ^^^^^^^^^^^^ no item named `read_exact` in scope
    |
    = help: to escape `[` and `]` characters, add '\' before them like `\[` or `\]`
error: aborting due to 2 previous errors

error: could not document `std`


Caused by:
  process didn't exit successfully: `/checkout/obj/build/bootstrap/debug/rustdoc --edition=2018 --crate-type dylib --crate-type rlib --crate-name std library/std/src/lib.rs --target x86_64-unknown-linux-gnu -o /checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/doc --cfg 'feature="addr2line"' --cfg 'feature="backtrace"' --cfg 'feature="compiler-builtins-c"' --cfg 'feature="gimli-symbolize"' --cfg 'feature="miniz_oxide"' --cfg 'feature="object"' --cfg 'feature="panic_unwind"' --cfg 'feature="std_detect_dlsym_getauxval"' --cfg 'feature="std_detect_file_io"' --error-format=json --json=diagnostic-rendered-ansi --markdown-css rust.css --markdown-no-toc -Z unstable-options --resource-suffix 1.51.0 --index-page /checkout/src/doc/index.md -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/release/deps --extern addr2line=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps/libaddr2line-6932873bb1bec35f.rmeta --extern alloc=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps/liballoc-9fcec9affb4bc249.rmeta --extern cfg_if=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps/libcfg_if-29e6f0a826659e36.rmeta --extern compiler_builtins=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps/libcompiler_builtins-cb5dc68e64b15c92.rmeta --extern core=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps/libcore-4e0337fd1372e206.rmeta --extern hashbrown=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps/libhashbrown-ffc6da5b2e764ec9.rmeta --extern libc=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps/liblibc-391366d606c00911.rmeta --extern miniz_oxide=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps/libminiz_oxide-d9cab8f63fb413fc.rmeta --extern object=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps/libobject-911638d33e88198d.rmeta --extern panic_abort=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps/libpanic_abort-de68e33796d2c4bb.rmeta --extern panic_unwind=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps/libpanic_unwind-297c4bf440e3cac1.rmeta --extern rustc_demangle=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps/librustc_demangle-097979bc5e3799f5.rmeta --extern unwind=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps/libunwind-eb520ca95bdb0049.rmeta --cfg=bootstrap -Dwarnings -Winvalid_codeblock_attributes --crate-version '1.51.0-nightly
  (626e43b5e
  2021-01-18)' --cfg backtrace_in_libstd` (exit code: 1)


command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustdoc" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "16" "--release" "--locked" "--color" "always" "--features" "panic-unwind backtrace compiler-builtins-c" "--manifest-path" "/checkout/library/test/Cargo.toml" "-p" "std" "--" "--markdown-css" "rust.css" "--markdown-no-toc" "-Z" "unstable-options" "--resource-suffix" "1.51.0" "--index-page" "/checkout/src/doc/index.md"


failed to run: /checkout/obj/build/bootstrap/debug/bootstrap doc --stage 0 library/std
Build completed unsuccessfully in 0:00:47
