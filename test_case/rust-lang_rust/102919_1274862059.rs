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
    Finished dev [unoptimized] target(s) in 0.05s
Documenting stage0 std (x86_64-unknown-linux-gnu) in HTML format
 Documenting core v0.0.0 (/checkout/library/core)
error[E0658]: the target feature `zksh` is currently unstable
   --> library/core/src/../../stdarch/crates/core_arch/src/riscv_shared/mod.rs:608:18
    |
608 | #[target_feature(enable = "zksh")]
    |
    = note: see issue #44839 <https://github.com/rust-lang/rust/issues/44839> for more information
    = note: see issue #44839 <https://github.com/rust-lang/rust/issues/44839> for more information
    = help: add `#![feature(riscv_target_feature)]` to the crate attributes to enable

error[E0658]: the target feature `zksh` is currently unstable
   --> library/core/src/../../stdarch/crates/core_arch/src/riscv_shared/mod.rs:638:18
    |
638 | #[target_feature(enable = "zksh")]
    |
    = note: see issue #44839 <https://github.com/rust-lang/rust/issues/44839> for more information
    = note: see issue #44839 <https://github.com/rust-lang/rust/issues/44839> for more information
    = help: add `#![feature(riscv_target_feature)]` to the crate attributes to enable

error[E0658]: the target feature `zksed` is currently unstable
   --> library/core/src/../../stdarch/crates/core_arch/src/riscv_shared/mod.rs:692:18
    |
692 | #[target_feature(enable = "zksed")]
    |
    = note: see issue #44839 <https://github.com/rust-lang/rust/issues/44839> for more information
    = note: see issue #44839 <https://github.com/rust-lang/rust/issues/44839> for more information
    = help: add `#![feature(riscv_target_feature)]` to the crate attributes to enable

error[E0658]: the target feature `zksed` is currently unstable
   --> library/core/src/../../stdarch/crates/core_arch/src/riscv_shared/mod.rs:752:18
    |
752 | #[target_feature(enable = "zksed")]
    |
    = note: see issue #44839 <https://github.com/rust-lang/rust/issues/44839> for more information
    = note: see issue #44839 <https://github.com/rust-lang/rust/issues/44839> for more information
    = help: add `#![feature(riscv_target_feature)]` to the crate attributes to enable
For more information about this error, try `rustc --explain E0658`.
error: could not document `core`

Caused by:
Caused by:
  process didn't exit successfully: `/checkout/obj/build/bootstrap/debug/rustdoc --edition=2021 --crate-type lib --crate-name core library/core/src/lib.rs --target x86_64-unknown-linux-gnu -o /checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/doc -Zunstable-options --check-cfg 'names()' --check-cfg 'values()' --error-format=json --json=diagnostic-rendered-ansi,artifacts,future-incompat -Z unstable-options --resource-suffix 1.66.0 --markdown-css rust.css --markdown-no-toc --index-page /checkout/src/doc/index.md -C metadata=abbb49baeba48eac -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/release/deps --cfg=bootstrap -Csymbol-mangling-version=legacy -Zunstable-options -Zunstable-options '--check-cfg=values(bootstrap)' '--check-cfg=values(stdarch_intel_sde)' '--check-cfg=values(no_fp_fmt_parse)' '--check-cfg=values(no_global_oom_handling)' '--check-cfg=values(no_rc)' '--check-cfg=values(no_sync)' '--check-cfg=values(freebsd12)' '--check-cfg=values(backtrace_in_libstd)' '--check-cfg=values(target_env,"libnx")' '--check-cfg=values(target_os,"watchos")' '--check-cfg=values(target_arch,"asmjs","spirv","nvptx","nvptx64","le32","xtensa")' '--check-cfg=values(dont_compile_me)' '--check-cfg=values(rustix_use_libc)' --document-private-items -Dwarnings '-Wrustdoc::invalid_codeblock_attributes' --crate-version '1.66.0-nightly
  (877aaa69e
  2022-10-11)' '-Zcrate-attr=doc(html_root_url="https://doc.rust-lang.org/nightly/")'` (exit status: 1)
