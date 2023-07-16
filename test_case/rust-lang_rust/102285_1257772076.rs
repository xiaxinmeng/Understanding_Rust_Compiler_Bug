plain
configure: rust.debug-assertions := True
configure: rust.overflow-checks := True
configure: llvm.assertions      := True
configure: dist.missing-tools   := True
configure: build.configure-args := ['--enable-sccache', '--disable-manage-submodu ...
configure: writing `config.toml` in current directory
configure: 
configure: run `python /checkout/x.py --help`
Attempting with retry: make prepare
---
 Documenting core v0.0.0 (/checkout/library/core)
error: unresolved link to `File`
   --> library/core/src/primitive_docs.rs:69:1
    |
69  | / /// The `!` type, also called "never".
70  | | ///
71  | | /// `!` represents the type of computations which never resolve to any value at all. For example,
72  | | /// the [`exit`] function `fn exit(code: i32) -> !` exits the process without ever returning, and
...   |
264 | | /// [`Debug`]: fmt::Debug
265 | | /// [`default()`]: Default::default
    |
    |
    = note: `-D rustdoc::broken-intra-doc-links` implied by `-D warnings`
    = note: the link appears in this line:
            
            Default` for (eg.) [`File`] by just making [`default()`] panic.)
    = note: no item named `File` in scope
    = note: no item named `File` in scope
    = help: to escape `[` and `]` characters, add '\' before them like `\[` or `\]`
error: unresolved link to `default`
   --> library/core/src/primitive_docs.rs:69:1
    |
    |
69  | / /// The `!` type, also called "never".
70  | | ///
71  | | /// `!` represents the type of computations which never resolve to any value at all. For example,
72  | | /// the [`exit`] function `fn exit(code: i32) -> !` exits the process without ever returning, and
...   |
264 | | /// [`Debug`]: fmt::Debug
265 | | /// [`default()`]: Default::default
    |
    = note: the link appears in this line:
            
            
            Default` for (eg.) [`File`] by just making [`default()`] panic.)
                                                        ^^^^^^^^^^^
    = help: to link to the module, prefix with `mod@`: mod@default
    = note: this link resolves to the module `default`, which is not in the value namespace
error: could not document `core`

Caused by:
Caused by:
  process didn't exit successfully: `/checkout/obj/build/bootstrap/debug/rustdoc --edition=2021 --crate-type lib --crate-name core library/core/src/lib.rs --target x86_64-unknown-linux-gnu -o /checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/doc -Zunstable-options --check-cfg 'names()' --check-cfg 'values()' --error-format=json --json=diagnostic-rendered-ansi,artifacts,future-incompat -Z unstable-options --resource-suffix 1.66.0 --markdown-css rust.css --markdown-no-toc --index-page /checkout/src/doc/index.md -C metadata=abbb49baeba48eac -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/release/deps --cfg=bootstrap -Csymbol-mangling-version=legacy -Zunstable-options -Zunstable-options '--check-cfg=values(bootstrap)' '--check-cfg=values(stdarch_intel_sde)' '--check-cfg=values(no_fp_fmt_parse)' '--check-cfg=values(no_global_oom_handling)' '--check-cfg=values(no_rc)' '--check-cfg=values(no_sync)' '--check-cfg=values(freebsd12)' '--check-cfg=values(backtrace_in_libstd)' '--check-cfg=values(target_env,"libnx")' '--check-cfg=values(target_os,"watchos")' '--check-cfg=values(target_arch,"asmjs","spirv","nvptx","nvptx64","le32","xtensa")' '--check-cfg=values(dont_compile_me)' --document-private-items -Dwarnings '-Wrustdoc::invalid_codeblock_attributes' --crate-version '1.66.0-nightly
  (ad4977e8a
  2022-09-25)' '-Zcrate-attr=doc(html_root_url="https://doc.rust-lang.org/nightly/")'` (exit status: 1)
