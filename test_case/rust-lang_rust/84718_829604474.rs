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
error: unresolved link to `std::env::set_current_dir`
   --> library/std/src/sys/unix/ext/fs.rs:893:6
    |
893 | /// [`std::env::set_current_dir`] afterwards.
    |      ^^^^^^^^^^^^^^^^^^^^^^^^^^^ no item named `std` in scope
    |
    = note: `-D rustdoc::broken-intra-doc-links` implied by `-D warnings`
error: unresolved link to `std::env::set_current_dir`
   --> library/std/src/sys/unix/ext/fs.rs:893:6
    |
    |
893 | /// [`std::env::set_current_dir`] afterwards.
    |      ^^^^^^^^^^^^^^^^^^^^^^^^^^^ no item named `std` in scope
error: aborting due to 2 previous errors

error: could not document `std`


Caused by:
  process didn't exit successfully: `/checkout/obj/build/bootstrap/debug/rustdoc --edition=2018 --crate-type dylib --crate-type rlib --crate-name std library/std/src/lib.rs --target x86_64-unknown-linux-gnu -o /checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/doc --cfg 'feature="addr2line"' --cfg 'feature="backtrace"' --cfg 'feature="compiler-builtins-c"' --cfg 'feature="gimli-symbolize"' --cfg 'feature="miniz_oxide"' --cfg 'feature="object"' --cfg 'feature="panic_unwind"' --cfg 'feature="std_detect_dlsym_getauxval"' --cfg 'feature="std_detect_file_io"' --error-format=json --json=diagnostic-rendered-ansi --markdown-css rust.css --markdown-no-toc -Z unstable-options --resource-suffix 1.53.0 --index-page /checkout/src/doc/index.md -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/release/deps --extern addr2line=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps/libaddr2line-672037617f10eb47.rmeta --extern alloc=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps/liballoc-07fb584f31d3af79.rmeta --extern cfg_if=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps/libcfg_if-02c297c5191dbb84.rmeta --extern compiler_builtins=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps/libcompiler_builtins-ab8b2622e21035fb.rmeta --extern core=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps/libcore-e600ef961286d3e8.rmeta --extern hashbrown=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps/libhashbrown-52aa9ba628b71f62.rmeta --extern libc=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps/liblibc-1a16476f49f19a8c.rmeta --extern miniz_oxide=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps/libminiz_oxide-e082f6375acf6643.rmeta --extern object=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps/libobject-577c1d7a68e373ef.rmeta --extern panic_abort=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps/libpanic_abort-1a761ed5437524d4.rmeta --extern panic_unwind=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps/libpanic_unwind-248fa6fd5f8fdcf7.rmeta --extern rustc_demangle=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps/librustc_demangle-b9aff77c610efe3c.rmeta --extern std_detect=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps/libstd_detect-986a0543d46ebbc0.rmeta --extern unwind=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps/libunwind-2891c2129ee84d3a.rmeta --cfg=bootstrap -Dwarnings '-Wrustdoc::invalid_codeblock_attributes' --crate-version '1.53.0-nightly
  (692b48700
  2021-04-29)' --cfg backtrace_in_libstd` (exit code: 1)


command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustdoc" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "16" "--release" "--locked" "--color" "always" "--features" "panic-unwind backtrace compiler-builtins-c" "--manifest-path" "/checkout/library/test/Cargo.toml" "-p" "std" "--" "--markdown-css" "rust.css" "--markdown-no-toc" "-Z" "unstable-options" "--resource-suffix" "1.53.0" "--index-page" "/checkout/src/doc/index.md"


failed to run: /checkout/obj/build/bootstrap/debug/bootstrap doc --stage 0 library/std
Build completed unsuccessfully in 0:00:55
