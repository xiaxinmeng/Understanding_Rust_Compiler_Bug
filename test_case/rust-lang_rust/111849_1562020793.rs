plain
    Checking core v0.0.0 (/checkout/library/core)
   Compiling compiler_builtins v0.1.91
    Checking rustc-std-workspace-core v1.99.0 (/checkout/library/rustc-std-workspace-core)
 Documenting alloc v0.0.0 (/checkout/library/alloc)
error: unresolved link to `UniqueRc::weak`
     |
     |
2795 |     /// Weak references to this `UniqueRc` can be created with [`UniqueRc::weak`]. Upgrading these
     |                                                                  ^^^^^^^^^^^^^^ the struct `UniqueRc` has no field or associated item named `weak`
     |
     = note: `-D rustdoc::broken-intra-doc-links` implied by `-D warnings`
error: could not document `alloc`

Caused by:
Caused by:
  process didn't exit successfully: `/checkout/obj/build/bootstrap/debug/rustdoc --edition=2021 --crate-type lib --crate-name alloc library/alloc/src/lib.rs --target x86_64-unknown-linux-gnu -o /checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/doc/x86_64-unknown-linux-gnu/doc --cfg 'feature="compiler-builtins-c"' -Zunstable-options --check-cfg 'names()' --check-cfg 'values()' --error-format=json --json=diagnostic-rendered-ansi,artifacts,future-incompat -Z unstable-options --resource-suffix 1.71.0 --markdown-css rust.css --markdown-no-toc --index-page /checkout/src/doc/index.md -C metadata=3ede8d0c18b54098 -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/doc/x86_64-unknown-linux-gnu/release/deps -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/doc/release/deps --extern compiler_builtins=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/doc/x86_64-unknown-linux-gnu/release/deps/libcompiler_builtins-82f50f274c4bfe7f.rmeta --extern core=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/doc/x86_64-unknown-linux-gnu/release/deps/libcore-77221b083c66e738.rmeta --cfg=bootstrap -Csymbol-mangling-version=legacy -Zunstable-options -Zunstable-options '--check-cfg=values(bootstrap)' '--check-cfg=values(stdarch_intel_sde)' '--check-cfg=values(no_fp_fmt_parse)' '--check-cfg=values(no_global_oom_handling)' '--check-cfg=values(no_rc)' '--check-cfg=values(no_sync)' '--check-cfg=values(freebsd12)' '--check-cfg=values(freebsd13)' '--check-cfg=values(backtrace_in_libstd)' '--check-cfg=values(target_env,"libnx")' '--check-cfg=values(target_arch,"asmjs","spirv","nvptx","xtensa")' --document-private-items --document-hidden-items -Dwarnings '-Wrustdoc::invalid_codeblock_attributes' --crate-version '1.71.0-nightly
  (681df1787
  2023-05-24)' '-Zcrate-attr=doc(html_root_url="https://doc.rust-lang.org/nightly/")' '-Zcrate-attr=warn(rust_2018_idioms)'` (exit status: 1)
