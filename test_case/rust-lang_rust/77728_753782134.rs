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
error: unresolved link to `1`
   --> library/std/src/sys/windows/ext/process.rs:104:67
    |
104 |     /// If the newly created process is a msys2/cygwin executable[1], the arg
    |                                                                   ^ no item named `1` in scope
    |
    = note: `-D broken-intra-doc-links` implied by `-D warnings`
    = help: to escape `[` and `]` characters, add '\' before them like `\[` or `\]`
error: unresolved link to `2`
   --> library/std/src/sys/windows/ext/process.rs:110:61
    |
    |
110 |     /// If the newly created process is a msvcrt executable[2], the commandline
    |                                                             ^ no item named `2` in scope
    |
    = help: to escape `[` and `]` characters, add '\' before them like `\[` or `\]`
error: unresolved link to `1`
   --> library/std/src/sys/windows/ext/process.rs:113:10
    |
    |
113 |     /// [1] <https://github.com/msys2/MSYS2-packages/issues/2176>
    |          ^ no item named `1` in scope
    |
    = help: to escape `[` and `]` characters, add '\' before them like `\[` or `\]`
error: unresolved link to `2`
   --> library/std/src/sys/windows/ext/process.rs:114:10
    |
    |
114 |     /// [2] <https://msdn.microsoft.com/en-us/library/17w5ykft.aspx>
    |          ^ no item named `2` in scope
    |
    = help: to escape `[` and `]` characters, add '\' before them like `\[` or `\]`
error: unresolved link to `1`
   --> library/std/src/sys/windows/ext/process.rs:104:67
    |
    |
104 |     /// If the newly created process is a msys2/cygwin executable[1], the arg
    |                                                                   ^ no item named `1` in scope
    |
    = help: to escape `[` and `]` characters, add '\' before them like `\[` or `\]`
error: aborting due to 5 previous errors

error: could not document `std`


Caused by:
  process didn't exit successfully: `/checkout/obj/build/bootstrap/debug/rustdoc --edition=2018 --crate-type dylib --crate-type rlib --crate-name std library/std/src/lib.rs --target x86_64-unknown-linux-gnu -o /checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/doc --cfg 'feature="addr2line"' --cfg 'feature="backtrace"' --cfg 'feature="compiler-builtins-c"' --cfg 'feature="gimli-symbolize"' --cfg 'feature="miniz_oxide"' --cfg 'feature="object"' --cfg 'feature="panic_unwind"' --cfg 'feature="std_detect_dlsym_getauxval"' --cfg 'feature="std_detect_file_io"' --error-format=json --json=diagnostic-rendered-ansi --markdown-css rust.css --markdown-no-toc -Z unstable-options --resource-suffix 1.51.0 --index-page /checkout/src/doc/index.md -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/release/deps --extern addr2line=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps/libaddr2line-f1b3a2dcceef7259.rmeta --extern alloc=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps/liballoc-35ea5e2e2a1ba305.rmeta --extern cfg_if=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps/libcfg_if-8dd4620629d7ffe2.rmeta --extern compiler_builtins=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps/libcompiler_builtins-b8bbfc17c24e8fc0.rmeta --extern core=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps/libcore-4e0337fd1372e206.rmeta --extern hashbrown=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps/libhashbrown-106fd252ea4a8fb8.rmeta --extern libc=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps/liblibc-391366d606c00911.rmeta --extern miniz_oxide=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps/libminiz_oxide-29c297135b93048a.rmeta --extern object=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps/libobject-bae264a36aacdb61.rmeta --extern panic_abort=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps/libpanic_abort-1921835201bc355d.rmeta --extern panic_unwind=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps/libpanic_unwind-492975535a47e92e.rmeta --extern rustc_demangle=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps/librustc_demangle-c56d65db42c4f7f3.rmeta --extern unwind=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps/libunwind-527993ab9311c91d.rmeta --cfg=bootstrap -Dwarnings -Winvalid_codeblock_attributes --crate-version '1.51.0-nightly
  (9dc6125e4
  2021-01-04)' --cfg backtrace_in_libstd` (exit code: 1)


command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustdoc" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "16" "--release" "--locked" "--color" "always" "--features" "panic-unwind backtrace compiler-builtins-c" "--manifest-path" "/checkout/library/test/Cargo.toml" "-p" "std" "--" "--markdown-css" "rust.css" "--markdown-no-toc" "-Z" "unstable-options" "--resource-suffix" "1.51.0" "--index-page" "/checkout/src/doc/index.md"


failed to run: /checkout/obj/build/bootstrap/debug/bootstrap doc --stage 0 library/std
Build completed unsuccessfully in 0:00:39
