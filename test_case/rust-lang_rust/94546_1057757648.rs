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
    Checking object v0.26.2
    Checking hashbrown v0.12.0
    Checking addr2line v0.16.0
 Documenting std v0.0.0 (/checkout/library/std)
error[E0658]: linking to associated items of raw pointers is experimental
    |
    |
448 | / /// Raw, unsafe pointers, `*const T`, and `*mut T`.
449 | | ///
450 | | /// *[See also the `std::ptr` module](ptr).*
...   |
...   |
554 | | /// [`drop`]: mem::drop
555 | | /// [`write`]: ptr::write
    |
    = note: see issue #80896 <https://github.com/rust-lang/rust/issues/80896> for more information
    = help: add `#![feature(intra_doc_pointers)]` to the crate attributes to enable
    = help: add `#![feature(intra_doc_pointers)]` to the crate attributes to enable
    = note: rustdoc does not allow disambiguating between `*const` and `*mut`, and pointers are unstable until it does

error[E0658]: linking to associated items of raw pointers is experimental
   |
   |
45 | /// [const-cast]: pointer::cast
   |
   = note: see issue #80896 <https://github.com/rust-lang/rust/issues/80896> for more information
   = help: add `#![feature(intra_doc_pointers)]` to the crate attributes to enable
   = help: add `#![feature(intra_doc_pointers)]` to the crate attributes to enable
   = note: rustdoc does not allow disambiguating between `*const` and `*mut`, and pointers are unstable until it does
For more information about this error, try `rustc --explain E0658`.
error: could not document `std`

Caused by:
Caused by:
  process didn't exit successfully: `/checkout/obj/build/bootstrap/debug/rustdoc --edition=2021 --crate-type dylib --crate-type rlib --crate-name std library/std/src/lib.rs --target x86_64-unknown-linux-gnu -o /checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/doc --cfg 'feature="addr2line"' --cfg 'feature="backtrace"' --cfg 'feature="compiler-builtins-c"' --cfg 'feature="gimli-symbolize"' --cfg 'feature="miniz_oxide"' --cfg 'feature="object"' --cfg 'feature="panic_unwind"' --cfg 'feature="std_detect_dlsym_getauxval"' --cfg 'feature="std_detect_file_io"' --error-format=json --json=diagnostic-rendered-ansi,artifacts,future-incompat --markdown-css rust.css --markdown-no-toc -Z unstable-options --resource-suffix 1.61.0 --index-page /checkout/src/doc/index.md -C metadata=641e0b646bb8c224 -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/release/deps --extern addr2line=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps/libaddr2line-de3910bf9159eaae.rmeta --extern alloc=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps/liballoc-7316f70c5e9c321f.rmeta --extern cfg_if=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps/libcfg_if-b9bd07f59430f24e.rmeta --extern compiler_builtins=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps/libcompiler_builtins-efdd225e54fd3917.rmeta --extern core=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps/libcore-07ea62845071a1d9.rmeta --extern hashbrown=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps/libhashbrown-9ca0b8109ef3244a.rmeta --extern libc=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps/liblibc-50baeda857587042.rmeta --extern miniz_oxide=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps/libminiz_oxide-fd9b4cca8ff80636.rmeta --extern object=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps/libobject-4fa5efe8fa010209.rmeta --extern panic_abort=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps/libpanic_abort-19b3bdd8d6d5eb52.rmeta --extern panic_unwind=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps/libpanic_unwind-454d52370fce988b.rmeta --extern rustc_demangle=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps/librustc_demangle-be3cd633eeb3034f.rmeta --extern std_detect=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps/libstd_detect-29256210cd4e31b3.rmeta --extern unwind=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps/libunwind-c00976d55deed0c0.rmeta --cfg=bootstrap -Csymbol-mangling-version=legacy -Zunstable-options -Dwarnings '-Wrustdoc::invalid_codeblock_attributes' --crate-version '1.61.0-nightly
  (71960e46d
  2022-03-03)' '-Zcrate-attr=doc(html_root_url="https://doc.rust-lang.org/nightly/")' --cfg backtrace_in_libstd` (exit status: 1)
