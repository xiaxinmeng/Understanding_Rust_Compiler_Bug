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
Building stage0 std artifacts (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
    Finished release [optimized] target(s) in 0.16s
Copying stage0 std from stage0 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu / x86_64-unknown-linux-gnu)
 Documenting core v0.0.0 (/checkout/library/core)
error: unresolved link to `self::ffi::CStr`
   |
   |
22 |           #[doc = include_str!($Docfile)]
...
...
49 | / type_alias! { "c_char.md", c_char = c_char_definition::c_char, NonZero_c_char = c_char_definition::NonZero_c_char;
50 | | // Make this type alias appear cfg-dependent so that Clippy does not suggest
51 | | // replacing `0 as c_char` with `0_i8`/`0_u8`. This #[cfg(all())] can be removed
52 | | // after the false positive in https://github.com/rust-lang/rust-clippy/issues/8093
53 | | // is fixed.
54 | | #[cfg(all())]
55 | | #[doc(cfg(all()))] }
   |
   |
   = note: `-D rustdoc::broken-intra-doc-links` implied by `-D warnings`
   = note: the link appears in this line:
           
           [`CStr`]: crate::ffi::CStr
   = note: no item named `CStr` in module `ffi`
   = note: no item named `CStr` in module `ffi`
   = note: this error originates in the macro `type_alias_no_nz` (in Nightly builds, run with -Z macro-backtrace for more info)
error: could not document `core`

Caused by:
Caused by:
  process didn't exit successfully: `/checkout/obj/build/bootstrap/debug/rustdoc --edition=2021 --crate-type lib --crate-name core library/core/src/lib.rs --target x86_64-unknown-linux-gnu -o /checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/doc --error-format=json --json=diagnostic-rendered-ansi,artifacts,future-incompat --markdown-css rust.css --markdown-no-toc -Z unstable-options --resource-suffix 1.61.0 --index-page /checkout/src/doc/index.md -C metadata=4599449636879869 -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/release/deps --cfg=bootstrap -Csymbol-mangling-version=legacy -Zunstable-options -Dwarnings '-Wrustdoc::invalid_codeblock_attributes' --crate-version '1.61.0-nightly
  (ff1b51418
  2022-03-02)' '-Zcrate-attr=doc(html_root_url="https://doc.rust-lang.org/nightly/")'` (exit status: 1)
