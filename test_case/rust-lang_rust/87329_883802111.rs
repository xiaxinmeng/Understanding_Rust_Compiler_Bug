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
    Checking std_detect v0.1.5 (/checkout/library/stdarch/crates/std_detect)
    Checking object v0.22.0
    Checking addr2line v0.14.0
 Documenting std v0.0.0 (/checkout/library/std)
error: unresolved link to `INVALID_HANDLE_VALUE`
  --> library/std/src/os/./windows/io/handle.rs:25:40
   |
25 | /// Note that it *may* have the value [`INVALID_HANDLE_VALUE`]. See [here] for
   |                                        ^^^^^^^^^^^^^^^^^^^^^^ no item named `INVALID_HANDLE_VALUE` in scope
   |
   = note: `-D rustdoc::broken-intra-doc-links` implied by `-D warnings`
   = help: to escape `[` and `]` characters, add '\' before them like `\[` or `\]`

error: unresolved link to `INVALID_HANDLE_VALUE`
  --> library/std/src/os/./windows/io/handle.rs:45:40
   |
45 | /// Note that it *may* have the value [`INVALID_HANDLE_VALUE`]. See [here] for
   |                                        ^^^^^^^^^^^^^^^^^^^^^^ no item named `INVALID_HANDLE_VALUE` in scope
   |
   = help: to escape `[` and `]` characters, add '\' before them like `\[` or `\]`

error: unresolved link to `INVALID_HANDLE_VALUE`
  --> library/std/src/os/./windows/io/handle.rs:58:12
   |
58 | /// where [`INVALID_HANDLE_VALUE`] is used as the sentry value, and null values
   |            ^^^^^^^^^^^^^^^^^^^^^^ no item named `INVALID_HANDLE_VALUE` in scope
   |
   = help: to escape `[` and `]` characters, add '\' before them like `\[` or `\]`

error: unresolved link to `INVALID_HANDLE_VALUE`
  --> library/std/src/os/./windows/io/handle.rs:66:6
   |
66 | /// [`INVALID_HANDLE_VALUE`] indicating an error or an otherwise absent value.
   |      ^^^^^^^^^^^^^^^^^^^^^^ no item named `INVALID_HANDLE_VALUE` in scope
   |
   = help: to escape `[` and `]` characters, add '\' before them like `\[` or `\]`

error: unresolved link to `INVALID_SOCKET`
  --> library/std/src/os/./windows/io/socket.rs:19:6
   |
19 | /// [`INVALID_SOCKET`].
   |      ^^^^^^^^^^^^^^^^ no item named `INVALID_SOCKET` in scope
   |
   = help: to escape `[` and `]` characters, add '\' before them like `\[` or `\]`

error: unresolved link to `INVALID_SOCKET`
  --> library/std/src/os/./windows/io/socket.rs:42:6
   |
42 | /// [`INVALID_SOCKET`].
   |      ^^^^^^^^^^^^^^^^ no item named `INVALID_SOCKET` in scope
   |
   = help: to escape `[` and `]` characters, add '\' before them like `\[` or `\]`

error: unresolved link to `INVALID_SOCKET`
  --> library/std/src/os/./windows/io/socket.rs:63:10
   |
63 |     /// [`INVALID_SOCKET`].
   |          ^^^^^^^^^^^^^^^^ no item named `INVALID_SOCKET` in scope
   |
   = help: to escape `[` and `]` characters, add '\' before them like `\[` or `\]`

error: unresolved link to `INVALID_HANDLE_VALUE`
  --> library/std/src/os/./windows/io/handle.rs:25:40
   |
25 | /// Note that it *may* have the value [`INVALID_HANDLE_VALUE`]. See [here] for
   |                                        ^^^^^^^^^^^^^^^^^^^^^^ no item named `INVALID_HANDLE_VALUE` in scope
   |
   = help: to escape `[` and `]` characters, add '\' before them like `\[` or `\]`

error: unresolved link to `INVALID_HANDLE_VALUE`
   --> library/std/src/os/./windows/io/handle.rs:192:19
    |
192 |     /// APIs use [`INVALID_HANDLE_VALUE`] for errors; see [here] for the full
    |                   ^^^^^^^^^^^^^^^^^^^^^^ no item named `INVALID_HANDLE_VALUE` in scope
    |
    = help: to escape `[` and `]` characters, add '\' before them like `\[` or `\]`
error: aborting due to 9 previous errors

error: could not document `std`


Caused by:
  process didn't exit successfully: `/checkout/obj/build/bootstrap/debug/rustdoc --edition=2018 --crate-type dylib --crate-type rlib --crate-name std library/std/src/lib.rs --target x86_64-unknown-linux-gnu -o /checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/doc --cfg 'feature="addr2line"' --cfg 'feature="backtrace"' --cfg 'feature="compiler-builtins-c"' --cfg 'feature="gimli-symbolize"' --cfg 'feature="miniz_oxide"' --cfg 'feature="object"' --cfg 'feature="panic_unwind"' --cfg 'feature="std_detect_dlsym_getauxval"' --cfg 'feature="std_detect_file_io"' --error-format=json --json=diagnostic-rendered-ansi --markdown-css rust.css --markdown-no-toc -Z unstable-options --resource-suffix 1.55.0 --index-page /checkout/src/doc/index.md -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/release/deps --extern addr2line=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps/libaddr2line-bd4393e75a62f459.rmeta --extern alloc=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps/liballoc-830f90728c968f04.rmeta --extern cfg_if=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps/libcfg_if-cb21ef14fc5b2a44.rmeta --extern compiler_builtins=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps/libcompiler_builtins-88a81bca321017bb.rmeta --extern core=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps/libcore-730f5e0d8d7f0fbb.rmeta --extern hashbrown=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps/libhashbrown-7874d60fc7400d88.rmeta --extern libc=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps/liblibc-723b91699d323eb0.rmeta --extern miniz_oxide=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps/libminiz_oxide-fe9185e44edb5104.rmeta --extern object=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps/libobject-f68001e3e025086a.rmeta --extern panic_abort=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps/libpanic_abort-137ee313240d7d56.rmeta --extern panic_unwind=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps/libpanic_unwind-d369bb2a3c548a06.rmeta --extern rustc_demangle=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps/librustc_demangle-7ad503bef1bf2383.rmeta --extern std_detect=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps/libstd_detect-ed639025922cf657.rmeta --extern unwind=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps/libunwind-b8ebc57b9187cf9f.rmeta --cfg=bootstrap -Dwarnings '-Wrustdoc::invalid_codeblock_attributes' --crate-version '1.55.0-nightly
  (cffcfd573
  2021-07-21)' '-Zcrate-attr=doc(html_root_url="https://doc.rust-lang.org/nightly/")' --cfg backtrace_in_libstd` (exit status: 1)


command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustdoc" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "16" "--release" "--locked" "--color" "always" "--features" "panic-unwind backtrace compiler-builtins-c" "--manifest-path" "/checkout/library/test/Cargo.toml" "-p" "std" "-Zskip-rustdoc-fingerprint" "--" "--markdown-css" "rust.css" "--markdown-no-toc" "-Z" "unstable-options" "--resource-suffix" "1.55.0" "--index-page" "/checkout/src/doc/index.md"


Build completed unsuccessfully in 0:00:29
