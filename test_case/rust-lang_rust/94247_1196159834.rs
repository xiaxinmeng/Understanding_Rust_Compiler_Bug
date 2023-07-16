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
error: unresolved link to `T`
    --> library/core/src/slice/iter.rs:1633:73
     |
1633 |     /// those requirements would mean that we could instead use a &mut [T] here, but we cannot
     |                                                                         ^ no item named `T` in scope
     |
     = note: `-D rustdoc::broken-intra-doc-links` implied by `-D warnings`
     = help: to escape `[` and `]` characters, add '\' before them like `\[` or `\]`
error: unresolved link to `T`
    --> library/core/src/slice/iter.rs:1634:64
     |
     |
1634 |     /// because __iterator_get_unchecked needs to return &mut [T], which guarantees certain aliasing
     |                                                                ^ no item named `T` in scope
     |
     = help: to escape `[` and `]` characters, add '\' before them like `\[` or `\]`
error: unresolved link to `T`
    --> library/core/src/slice/iter.rs:1635:83
     |
     |
1635 |     /// properties that we cannot uphold if we hold on to the full original &mut [T]. Wrapping a raw
     |                                                                                   ^ no item named `T` in scope
     |
     = help: to escape `[` and `]` characters, add '\' before them like `\[` or `\]`
error: unresolved link to `T`
    --> library/core/src/slice/iter.rs:1636:62
     |
     |
1636 |     /// slice instead lets us hand out non-overlapping &mut [T] subslices of the slice we wrap.
     |                                                              ^ no item named `T` in scope
     |
     = help: to escape `[` and `]` characters, add '\' before them like `\[` or `\]`
error: unresolved link to `T`
    --> library/core/src/slice/iter.rs:1975:73
     |
     |
1975 |     /// those requirements would mean that we could instead use a &mut [T] here, but we cannot
     |                                                                         ^ no item named `T` in scope
     |
     = help: to escape `[` and `]` characters, add '\' before them like `\[` or `\]`
error: unresolved link to `T`
    --> library/core/src/slice/iter.rs:1976:64
     |
     |
1976 |     /// because __iterator_get_unchecked needs to return &mut [T], which guarantees certain aliasing
     |                                                                ^ no item named `T` in scope
     |
     = help: to escape `[` and `]` characters, add '\' before them like `\[` or `\]`
error: unresolved link to `T`
    --> library/core/src/slice/iter.rs:1977:83
     |
     |
1977 |     /// properties that we cannot uphold if we hold on to the full original &mut [T]. Wrapping a raw
     |                                                                                   ^ no item named `T` in scope
     |
     = help: to escape `[` and `]` characters, add '\' before them like `\[` or `\]`
error: unresolved link to `T`
    --> library/core/src/slice/iter.rs:1978:62
     |
     |
1978 |     /// slice instead lets us hand out non-overlapping &mut [T] subslices of the slice we wrap.
     |                                                              ^ no item named `T` in scope
     |
     = help: to escape `[` and `]` characters, add '\' before them like `\[` or `\]`
error: unresolved link to `T`
    --> library/core/src/slice/iter.rs:2675:73
     |
     |
2675 |     /// those requirements would mean that we could instead use a &mut [T] here, but we cannot
     |                                                                         ^ no item named `T` in scope
     |
     = help: to escape `[` and `]` characters, add '\' before them like `\[` or `\]`
error: unresolved link to `T`
    --> library/core/src/slice/iter.rs:2676:64
     |
     |
2676 |     /// because __iterator_get_unchecked needs to return &mut [T], which guarantees certain aliasing
     |                                                                ^ no item named `T` in scope
     |
     = help: to escape `[` and `]` characters, add '\' before them like `\[` or `\]`
error: unresolved link to `T`
    --> library/core/src/slice/iter.rs:2677:83
     |
     |
2677 |     /// properties that we cannot uphold if we hold on to the full original &mut [T]. Wrapping a raw
     |                                                                                   ^ no item named `T` in scope
     |
     = help: to escape `[` and `]` characters, add '\' before them like `\[` or `\]`
error: unresolved link to `T`
    --> library/core/src/slice/iter.rs:2678:62
     |
     |
2678 |     /// slice instead lets us hand out non-overlapping &mut [T] subslices of the slice we wrap.
     |                                                              ^ no item named `T` in scope
     |
     = help: to escape `[` and `]` characters, add '\' before them like `\[` or `\]`
error: unresolved link to `T`
    --> library/core/src/slice/iter.rs:3025:73
     |
     |
3025 |     /// those requirements would mean that we could instead use a &mut [T] here, but we cannot
     |                                                                         ^ no item named `T` in scope
     |
     = help: to escape `[` and `]` characters, add '\' before them like `\[` or `\]`
error: unresolved link to `T`
    --> library/core/src/slice/iter.rs:3026:64
     |
     |
3026 |     /// because __iterator_get_unchecked needs to return &mut [T], which guarantees certain aliasing
     |                                                                ^ no item named `T` in scope
     |
     = help: to escape `[` and `]` characters, add '\' before them like `\[` or `\]`
error: unresolved link to `T`
    --> library/core/src/slice/iter.rs:3027:83
     |
     |
3027 |     /// properties that we cannot uphold if we hold on to the full original &mut [T]. Wrapping a raw
     |                                                                                   ^ no item named `T` in scope
     |
     = help: to escape `[` and `]` characters, add '\' before them like `\[` or `\]`
error: unresolved link to `T`
    --> library/core/src/slice/iter.rs:3028:62
     |
     |
3028 |     /// slice instead lets us hand out non-overlapping &mut [T] subslices of the slice we wrap.
     |                                                              ^ no item named `T` in scope
     |
     = help: to escape `[` and `]` characters, add '\' before them like `\[` or `\]`
error: could not document `core`

Caused by:
Caused by:
  process didn't exit successfully: `/checkout/obj/build/bootstrap/debug/rustdoc --edition=2021 --crate-type lib --crate-name core library/core/src/lib.rs --target x86_64-unknown-linux-gnu -o /checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/doc -Zunstable-options --check-cfg 'names()' --check-cfg 'values()' --error-format=json --json=diagnostic-rendered-ansi,artifacts,future-incompat --markdown-css rust.css --markdown-no-toc -Z unstable-options --resource-suffix 1.64.0 --index-page /checkout/src/doc/index.md -C metadata=abbb49baeba48eac -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/release/deps --cfg=bootstrap -Csymbol-mangling-version=legacy -Zunstable-options -Zunstable-options '--check-cfg=values(bootstrap)' '--check-cfg=values(stdarch_intel_sde)' '--check-cfg=values(no_fp_fmt_parse)' '--check-cfg=values(no_global_oom_handling)' '--check-cfg=values(freebsd12)' '--check-cfg=values(backtrace_in_libstd)' '--check-cfg=values(target_env,"libnx")' '--check-cfg=values(target_os,"watchos")' '--check-cfg=values(target_arch,"asmjs","spirv","nvptx","nvptx64","le32","xtensa")' '--check-cfg=values(dont_compile_me)' -Dwarnings '-Wrustdoc::invalid_codeblock_attributes' --crate-version '1.64.0-nightly
  (12500add5
  2022-07-27)' '-Zcrate-attr=doc(html_root_url="https://doc.rust-lang.org/nightly/")'` (exit status: 1)
