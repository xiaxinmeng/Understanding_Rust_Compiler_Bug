plain
configure: rust.debug-assertions := True
configure: rust.overflow-checks := True
configure: llvm.assertions      := True
configure: dist.missing-tools   := True
configure: build.configure-args := ['--enable-sccache', '--disable-manage-submodu ...
configure: writing `config.toml` in current directory
configure: 
configure: run `python /checkout/x.py --help`
configure: 
---
    Checking std_detect v0.1.5 (/checkout/library/stdarch/crates/std_detect)
    Checking object v0.26.2
    Checking addr2line v0.16.0
 Documenting std v0.0.0 (/checkout/library/std)
error[E0432]: unresolved imports `crate::os::windows::io::FromHandle`, `crate::os::windows::io::FromSocket`, `crate::os::windows::io::IntoHandle`, `crate::os::windows::io::IntoSocket`
 --> library/std/src/os/windows/io/raw.rs:9:50
  |
9 | use crate::os::windows::io::{AsHandle, AsSocket, FromHandle, FromSocket, IntoHandle, IntoSocket};
  |                                                  ^^^^^^^^^^  ^^^^^^^^^^  ^^^^^^^^^^  ^^^^^^^^^^ no `IntoSocket` in `os::windows::io`
  |                                                  |           |           |
  |                                                  |           |           no `IntoHandle` in `os::windows::io`
  |                                                  |           no `FromSocket` in `os::windows::io`
  |                                                  no `FromHandle` in `os::windows::io`
help: a similar name exists in the module
  |
  |
9 | use crate::os::windows::io::{AsHandle, AsSocket, FromRawHandle, FromSocket, IntoHandle, IntoSocket};
help: a similar name exists in the module
  |
  |
9 | use crate::os::windows::io::{AsHandle, AsSocket, FromHandle, FromRawSocket, IntoHandle, IntoSocket};
help: a similar name exists in the module
  |
  |
9 | use crate::os::windows::io::{AsHandle, AsSocket, FromHandle, FromSocket, IntoRawHandle, IntoSocket};
help: a similar name exists in the module
  |
  |
9 | use crate::os::windows::io::{AsHandle, AsSocket, FromHandle, FromSocket, IntoHandle, IntoRawSocket};


error[E0432]: unresolved imports `crate::os::unix::io::FromFd`, `crate::os::unix::io::IntoFd`
   |
   |
11 | use crate::os::unix::io::{AsFd, FromFd, IntoFd};
   |                                 ^^^^^^  ^^^^^^ no `IntoFd` in `os::unix::io`
   |                                 |
   |                                 no `FromFd` in `os::unix::io`
error: Compilation failed, aborting rustdoc

For more information about this error, try `rustc --explain E0432`.
error: could not document `std`
error: could not document `std`

Caused by:
  process didn't exit successfully: `/checkout/obj/build/bootstrap/debug/rustdoc --edition=2021 --crate-type dylib --crate-type rlib --crate-name std library/std/src/lib.rs --target x86_64-unknown-linux-gnu -o /checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/doc --cfg 'feature="addr2line"' --cfg 'feature="backtrace"' --cfg 'feature="compiler-builtins-c"' --cfg 'feature="gimli-symbolize"' --cfg 'feature="miniz_oxide"' --cfg 'feature="object"' --cfg 'feature="panic_unwind"' --cfg 'feature="std_detect_dlsym_getauxval"' --cfg 'feature="std_detect_file_io"' --error-format=json --json=diagnostic-rendered-ansi,future-incompat --markdown-css rust.css --markdown-no-toc -Z unstable-options --resource-suffix 1.60.0 --index-page /checkout/src/doc/index.md -C metadata=6bdbe3ef84a518db -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/release/deps --extern addr2line=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps/libaddr2line-e263bc4ca29a612a.rmeta --extern alloc=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps/liballoc-29279582eba47b31.rmeta --extern cfg_if=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps/libcfg_if-2eedcb83854cdbd3.rmeta --extern compiler_builtins=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps/libcompiler_builtins-cdcf79e32210781f.rmeta --extern core=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps/libcore-07ea62845071a1d9.rmeta --extern hashbrown=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps/libhashbrown-4db7a586621dcf3a.rmeta --extern libc=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps/liblibc-50baeda857587042.rmeta --extern miniz_oxide=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps/libminiz_oxide-b4d2f3aa6071f051.rmeta --extern object=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps/libobject-90e73292a998c47f.rmeta --extern panic_abort=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps/libpanic_abort-57daebc74b78b2c0.rmeta --extern panic_unwind=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps/libpanic_unwind-4ef7f2196bfbb5a6.rmeta --extern rustc_demangle=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps/librustc_demangle-946a82efe12cb56a.rmeta --extern std_detect=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps/libstd_detect-2109cff20159681b.rmeta --extern unwind=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps/libunwind-daf63e3586fbd030.rmeta --cfg=bootstrap -Csymbol-mangling-version=legacy -Zunstable-options -Dwarnings '-Wrustdoc::invalid_codeblock_attributes' --crate-version '1.60.0-nightly
  (58a0bb28c
  2022-02-02)' '-Zcrate-attr=doc(html_root_url="https://doc.rust-lang.org/nightly/")' --cfg backtrace_in_libstd` (exit status: 1)
