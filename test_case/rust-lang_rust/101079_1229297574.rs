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
error: unresolved link to `mem::size_of_val`
   --> library/core/src/intrinsics.rs:899:55
    |
899 |     /// The stabilized version of this intrinsic is [`mem::size_of_val`].
    |                                                       ^^^^^^^^^^^^^^^^ no item named `mem` in scope
    |
    = note: `-D rustdoc::broken-intra-doc-links` implied by `-D warnings`
error: unresolved link to `mem::forget_unsized`
   --> library/core/src/intrinsics.rs:966:34
    |
    |
966 |     /// This exists solely for [`mem::forget_unsized`]; normal `forget` uses
    |                                  ^^^^^^^^^^^^^^^^^^^ no item named `mem` in scope
error: unresolved link to `mem::MaybeUninit`
    --> library/core/src/intrinsics.rs:1087:60
     |
     |
1087 |     ///   type for that buffer, it can use [`MaybeUninit`][mem::MaybeUninit].
     |                                                            ^^^^^^^^^^^^^^^^ no item named `mem` in scope
error: unresolved link to `mem::variant_count`
    --> library/core/src/intrinsics.rs:1976:61
     |
     |
1976 |     /// The to-be-stabilized version of this intrinsic is [`mem::variant_count`].
     |                                                             ^^^^^^^^^^^^^^^^^^ no item named `mem` in scope
error: could not document `core`

Caused by:
Caused by:
  process didn't exit successfully: `/checkout/obj/build/bootstrap/debug/rustdoc --edition=2021 --crate-type lib --crate-name core library/core/src/lib.rs --target x86_64-unknown-linux-gnu -o /checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/doc -Zunstable-options --check-cfg 'names()' --check-cfg 'values()' --error-format=json --json=diagnostic-rendered-ansi,artifacts,future-incompat --markdown-css rust.css --markdown-no-toc -Z unstable-options --resource-suffix 1.65.0 --index-page /checkout/src/doc/index.md -C metadata=abbb49baeba48eac -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/release/deps --cfg=bootstrap -Csymbol-mangling-version=legacy -Zunstable-options -Zunstable-options '--check-cfg=values(bootstrap)' '--check-cfg=values(stdarch_intel_sde)' '--check-cfg=values(no_fp_fmt_parse)' '--check-cfg=values(no_global_oom_handling)' '--check-cfg=values(freebsd12)' '--check-cfg=values(backtrace_in_libstd)' '--check-cfg=values(target_env,"libnx")' '--check-cfg=values(target_os,"watchos")' '--check-cfg=values(target_arch,"asmjs","spirv","nvptx","nvptx64","le32","xtensa")' '--check-cfg=values(dont_compile_me)' -Dwarnings '-Wrustdoc::invalid_codeblock_attributes' --crate-version '1.65.0-nightly
  (d6cabfcf7
  2022-08-27)' '-Zcrate-attr=doc(html_root_url="https://doc.rust-lang.org/nightly/")'` (exit status: 1)
