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
Building rustbuild
    Finished dev [unoptimized] target(s) in 0.06s
Documenting {test} stage0 library (x86_64-unknown-linux-gnu) in HTML format
 Documenting core v0.0.0 (/checkout/library/core)
error: unresolved link to `reduce`
     |
     |
2539 |     /// For iterators with at least one element, this is the same as [`reduce()`]
     |                                                                        ^^^^^^^^ no item named `reduce` in scope
     |
     = help: to escape `[` and `]` characters, add '\' before them like `\[` or `\]`
     = note: `-D rustdoc::broken-intra-doc-links` implied by `-D warnings`
error: could not document `core`

Caused by:
Caused by:
  process didn't exit successfully: `/checkout/obj/build/bootstrap/debug/rustdoc --edition=2021 --crate-type lib --crate-name core library/core/src/lib.rs --target x86_64-unknown-linux-gnu -o /checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/doc/x86_64-unknown-linux-gnu/doc -Zunstable-options --check-cfg 'names()' --check-cfg 'values()' --error-format=json --json=diagnostic-rendered-ansi,artifacts,future-incompat -Z unstable-options --resource-suffix 1.68.0 --markdown-css rust.css --markdown-no-toc --index-page /checkout/src/doc/index.md -C metadata=abbb49baeba48eac -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/doc/x86_64-unknown-linux-gnu/release/deps -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/doc/release/deps --cfg=bootstrap -Csymbol-mangling-version=legacy -Zunstable-options -Zunstable-options '--check-cfg=values(bootstrap)' '--check-cfg=values(stdarch_intel_sde)' '--check-cfg=values(no_fp_fmt_parse)' '--check-cfg=values(no_global_oom_handling)' '--check-cfg=values(no_rc)' '--check-cfg=values(no_sync)' '--check-cfg=values(freebsd12)' '--check-cfg=values(backtrace_in_libstd)' '--check-cfg=values(target_env,"libnx")' '--check-cfg=values(target_os,"watchos")' '--check-cfg=values(target_arch,"asmjs","spirv","nvptx","nvptx64","le32","xtensa")' '--check-cfg=values(dont_compile_me)' '--check-cfg=values(rustix_use_libc)' --document-private-items --document-hidden-items -Dwarnings '-Wrustdoc::invalid_codeblock_attributes' --crate-version '1.68.0-nightly
  (f2bd6e1ed
  2023-01-01)' '-Zcrate-attr=doc(html_root_url="https://doc.rust-lang.org/nightly/")'` (exit status: 1)
