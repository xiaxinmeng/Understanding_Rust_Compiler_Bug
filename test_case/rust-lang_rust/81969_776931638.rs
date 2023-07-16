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
    Checking miniz_oxide v0.4.0
    Checking hashbrown v0.9.0
    Checking addr2line v0.14.0
 Documenting std v0.0.0 (/checkout/library/std)
error[E0432]: unresolved imports `crate::os::windows::io::AsRawHandle`, `crate::os::windows::io::FromRawHandle`, `crate::os::windows::io::IntoRawHandle`, `crate::os::windows::io::RawHandle`
 --> library/std/src/sys/windows/ext/process.rs:5:30
  |
5 | use crate::os::windows::io::{AsRawHandle, FromRawHandle, IntoRawHandle, RawHandle};
  |                              ^^^^^^^^^^^  ^^^^^^^^^^^^^  ^^^^^^^^^^^^^  ^^^^^^^^^ no `RawHandle` in `sys::unix::ext::io`
  |                              |            |              |
  |                              |            |              no `IntoRawHandle` in `sys::unix::ext::io`
  |                              |            no `FromRawHandle` in `sys::unix::ext::io`
  |                              no `AsRawHandle` in `sys::unix::ext::io`

error[E0432]: unresolved imports `crate::os::windows::io::AsRawHandle`, `crate::os::windows::io::IntoRawHandle`, `crate::os::windows::io::RawHandle`
 --> library/std/src/sys/windows/ext/thread.rs:5:30
  |
5 | use crate::os::windows::io::{AsRawHandle, IntoRawHandle, RawHandle};
  |                              ^^^^^^^^^^^  ^^^^^^^^^^^^^  ^^^^^^^^^ no `RawHandle` in `sys::unix::ext::io`
  |                              |            |
  |                              |            no `IntoRawHandle` in `sys::unix::ext::io`
  |                              no `AsRawHandle` in `sys::unix::ext::io`

error[E0412]: cannot find type `SOCKET` in module `crate::os::windows::raw`
  --> library/std/src/sys/windows/c.rs:62:44
   |
62 | pub type SOCKET = crate::os::windows::raw::SOCKET;
   |                                            ^^^^^^ not found in `crate::os::windows::raw`
help: consider importing this type alias
   |
   |
7  | use crate::sys::windows_ext::raw::SOCKET;


error[E0412]: cannot find type `HANDLE` in module `raw`
  --> library/std/src/sys/windows/ext/io.rs:15:27
   |
15 | pub type RawHandle = raw::HANDLE;
   |                           ^^^^^^ not found in `raw`
help: consider importing one of these items
   |
   |
5  | use crate::sys::c::HANDLE;
5  | use crate::sys::windows_ext::raw::HANDLE;
   |


error[E0412]: cannot find type `SOCKET` in module `raw`
  --> library/std/src/sys/windows/ext/io.rs:19:27
   |
19 | pub type RawSocket = raw::SOCKET;
   |                           ^^^^^^ not found in `raw`
help: consider importing one of these items
   |
   |
5  | use crate::sys::c::SOCKET;
   |
5  | use crate::sys::windows_ext::raw::SOCKET;

error: Compilation failed, aborting rustdoc

error: aborting due to 6 previous errors
error: aborting due to 6 previous errors

Some errors have detailed explanations: E0412, E0432.
For more information about an error, try `rustc --explain E0412`.
error: could not document `std`

Caused by:
  process didn't exit successfully: `/checkout/obj/build/bootstrap/debug/rustdoc --edition=2018 --crate-type dylib --crate-type rlib --crate-name std library/std/src/lib.rs --target x86_64-unknown-linux-gnu -o /checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/doc --cfg 'feature="addr2line"' --cfg 'feature="backtrace"' --cfg 'feature="compiler-builtins-c"' --cfg 'feature="gimli-symbolize"' --cfg 'feature="miniz_oxide"' --cfg 'feature="object"' --cfg 'feature="panic_unwind"' --cfg 'feature="std_detect_dlsym_getauxval"' --cfg 'feature="std_detect_file_io"' --error-format=json --json=diagnostic-rendered-ansi --markdown-css rust.css --markdown-no-toc -Z unstable-options --resource-suffix 1.52.0 --index-page /checkout/src/doc/index.md -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/release/deps --extern addr2line=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps/libaddr2line-6932873bb1bec35f.rmeta --extern alloc=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps/liballoc-9fcec9affb4bc249.rmeta --extern cfg_if=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps/libcfg_if-29e6f0a826659e36.rmeta --extern compiler_builtins=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps/libcompiler_builtins-cb5dc68e64b15c92.rmeta --extern core=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps/libcore-4e0337fd1372e206.rmeta --extern hashbrown=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps/libhashbrown-ffc6da5b2e764ec9.rmeta --extern libc=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps/liblibc-a5b98d1584506141.rmeta --extern miniz_oxide=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps/libminiz_oxide-d9cab8f63fb413fc.rmeta --extern object=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps/libobject-911638d33e88198d.rmeta --extern panic_abort=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps/libpanic_abort-87d775de5900d430.rmeta --extern panic_unwind=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps/libpanic_unwind-6d6ed8527afc0135.rmeta --extern rustc_demangle=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps/librustc_demangle-097979bc5e3799f5.rmeta --extern unwind=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps/libunwind-90d03f674cde209c.rmeta --cfg=bootstrap -Dwarnings -Winvalid_codeblock_attributes --crate-version '1.52.0-nightly
  (d85cbfcc6
  2021-02-10)' --cfg backtrace_in_libstd` (exit code: 1)


command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustdoc" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "16" "--release" "--locked" "--color" "always" "--features" "panic-unwind backtrace compiler-builtins-c" "--manifest-path" "/checkout/library/test/Cargo.toml" "-p" "std" "--" "--markdown-css" "rust.css" "--markdown-no-toc" "-Z" "unstable-options" "--resource-suffix" "1.52.0" "--index-page" "/checkout/src/doc/index.md"


failed to run: /checkout/obj/build/bootstrap/debug/bootstrap doc --stage 0 library/std
Build completed unsuccessfully in 0:00:40
