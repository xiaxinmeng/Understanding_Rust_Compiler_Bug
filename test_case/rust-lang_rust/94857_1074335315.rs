plain
    Checking hashbrown v0.12.0
    Checking miniz_oxide v0.4.0
    Checking addr2line v0.16.0
 Documenting std v0.0.0 (/checkout/library/std)
no resolution for "Self::Normal" MacroNS DefId(1:277 ~ core[3b50]::num)
no resolution for "Self::Normal" TypeNS DefId(1:277 ~ core[3b50]::num)
no resolution for "Self::Normal" ValueNS DefId(1:277 ~ core[3b50]::num)
no resolution for "Self::Zero" MacroNS DefId(1:277 ~ core[3b50]::num)
no resolution for "Self::Zero" TypeNS DefId(1:277 ~ core[3b50]::num)
no resolution for "Self::Zero" ValueNS DefId(1:277 ~ core[3b50]::num)
no resolution for "f32::MIN_POSITIVE" MacroNS DefId(1:277 ~ core[3b50]::num)
no resolution for "f32::MIN_POSITIVE" TypeNS DefId(1:277 ~ core[3b50]::num)
no resolution for "f32::MIN_POSITIVE" ValueNS DefId(1:277 ~ core[3b50]::num)
no resolution for "f64::MIN_POSITIVE" MacroNS DefId(1:277 ~ core[3b50]::num)
no resolution for "f64::MIN_POSITIVE" TypeNS DefId(1:277 ~ core[3b50]::num)
no resolution for "f64::MIN_POSITIVE" ValueNS DefId(1:277 ~ core[3b50]::num)
no resolution for "f32::MAX" MacroNS DefId(1:277 ~ core[3b50]::num)
no resolution for "f32::MAX" TypeNS DefId(1:277 ~ core[3b50]::num)
no resolution for "f32::MAX" ValueNS DefId(1:277 ~ core[3b50]::num)
no resolution for "f64::MAX" MacroNS DefId(1:277 ~ core[3b50]::num)
no resolution for "f64::MAX" TypeNS DefId(1:277 ~ core[3b50]::num)
no resolution for "f64::MAX" ValueNS DefId(1:277 ~ core[3b50]::num)
    Checking std v0.0.0 (/checkout/library/std)
 Documenting proc_macro v0.0.0 (/checkout/library/proc_macro)
 Documenting proc_macro v0.0.0 (/checkout/library/proc_macro)
error[E0773]: attempted to define built-in macro more than once
     |
1311 | /     macro_rules! cfg {
1311 | /     macro_rules! cfg {
1312 | |         ($($cfg:tt)*) => {
1314 | |         };
1315 | |     }
     | |_____^
     |
     |
note: previously defined here
    --> /checkout/library/core/src/macros/mod.rs:1311:5
     |
1311 | /     macro_rules! cfg {
1312 | |         ($($cfg:tt)*) => {
1314 | |         };
1315 | |     }
     | |_____^


error: Compilation failed, aborting rustdoc

For more information about this error, try `rustc --explain E0773`.
error: could not document `proc_macro`

Caused by:
  process didn't exit successfully: `/checkout/obj/build/bootstrap/debug/rustdoc --edition=2021 --crate-type lib --crate-name proc_macro library/proc_macro/src/lib.rs --target x86_64-unknown-linux-gnu -o /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-unknown-linux-gnu/doc --error-format=json --json=diagnostic-rendered-ansi,artifacts,future-incompat --markdown-css rust.css --markdown-no-toc -Z unstable-options --resource-suffix 1.61.0 --index-page /checkout/src/doc/index.md -C metadata=0e5a90eca7c06845 -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-unknown-linux-gnu/release/deps -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/release/deps --extern std=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-unknown-linux-gnu/release/deps/libstd-32b0c990c1e4f479.rmeta -Csymbol-mangling-version=legacy -Zunstable-options -Zunstable-options '--check-cfg=names()' '--check-cfg=values()' '--check-cfg=values(bootstrap)' '--check-cfg=values(stdarch_intel_sde)' '--check-cfg=values(no_fp_fmt_parse)' '--check-cfg=values(no_global_oom_handling)' '--check-cfg=values(freebsd12)' '--check-cfg=values(backtrace_in_libstd)' '--check-cfg=values(target_env,"libnx")' '--check-cfg=values(target_os,"watchos")' '--check-cfg=values(target_arch,"asmjs","spirv","nvptx","nvptx64","le32","xtensa")' '--check-cfg=values(dont_compile_me)' -Dwarnings '-Wrustdoc::invalid_codeblock_attributes' --crate-version '1.61.0-nightly
  (2ecc3560b
  2022-03-21)' '-Zcrate-attr=doc(html_root_url="https://doc.rust-lang.org/nightly/")'` (exit status: 1)
