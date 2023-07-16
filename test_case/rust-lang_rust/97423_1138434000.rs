plain
Documenting standalone (x86_64-unknown-linux-gnu)
Documenting book redirect pages (x86_64-unknown-linux-gnu)
Documenting stage2 std (x86_64-unknown-linux-gnu)
 Documenting core v0.0.0 (/checkout/library/core)
error: unresolved link to `Ordering::Acqrel`
    |
    |
352 |     /// [`Ordering::Acqrel`] and [`Ordering::Relaxed`] as the success and failure parameters.
    |           ^^^^^^^^^^^^^^^^ the enum `Ordering` has no variant or associated item named `Acqrel`
    |
    = note: `-D rustdoc::broken-intra-doc-links` implied by `-D warnings`

error: unresolved link to `Ordering::Acqrel`
    |
    |
359 |     /// [`Ordering::Acqrel`] and [`Ordering::Acquire`] as the success and failure parameters.
    |           ^^^^^^^^^^^^^^^^ the enum `Ordering` has no variant or associated item named `Acqrel`

error: unresolved link to `Ordering::Acqrel`
    |
    |
366 |     /// [`Ordering::Acqrel`] and [`Ordering::SeqCst`] as the success and failure parameters.
    |           ^^^^^^^^^^^^^^^^ the enum `Ordering` has no variant or associated item named `Acqrel`

error: unresolved link to `Ordering::Acqrel`
    |
    |
458 |     /// [`Ordering::Acqrel`] and [`Ordering::Relaxed`] as the success and failure parameters.
    |           ^^^^^^^^^^^^^^^^ the enum `Ordering` has no variant or associated item named `Acqrel`

error: unresolved link to `Ordering::Acqrel`
    |
    |
465 |     /// [`Ordering::Acqrel`] and [`Ordering::Acquire`] as the success and failure parameters.
    |           ^^^^^^^^^^^^^^^^ the enum `Ordering` has no variant or associated item named `Acqrel`

error: unresolved link to `Ordering::Acqrel`
    |
    |
472 |     /// [`Ordering::Acqrel`] and [`Ordering::SeqCst`] as the success and failure parameters.
    |           ^^^^^^^^^^^^^^^^ the enum `Ordering` has no variant or associated item named `Acqrel`
error: could not document `core`

Caused by:
Caused by:
  process didn't exit successfully: `/checkout/obj/build/bootstrap/debug/rustdoc --edition=2021 --crate-type lib --crate-name core library/core/src/lib.rs --target x86_64-unknown-linux-gnu -o /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-unknown-linux-gnu/doc --error-format=json --json=diagnostic-rendered-ansi,artifacts,future-incompat --markdown-css rust.css --markdown-no-toc -Z unstable-options --resource-suffix 1.63.0 --index-page /checkout/src/doc/index.md -C metadata=f4ae526a78b232e4 -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-unknown-linux-gnu/release/deps -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/release/deps -Csymbol-mangling-version=legacy -Zunstable-options -Zunstable-options '--check-cfg=names()' '--check-cfg=values()' '--check-cfg=values(bootstrap)' '--check-cfg=values(stdarch_intel_sde)' '--check-cfg=values(no_fp_fmt_parse)' '--check-cfg=values(no_global_oom_handling)' '--check-cfg=values(freebsd12)' '--check-cfg=values(backtrace_in_libstd)' '--check-cfg=values(target_env,"libnx")' '--check-cfg=values(target_os,"watchos")' '--check-cfg=values(target_arch,"asmjs","spirv","nvptx","nvptx64","le32","xtensa")' '--check-cfg=values(dont_compile_me)' -Dwarnings '-Wrustdoc::invalid_codeblock_attributes' --crate-version '1.63.0-nightly
  (c613b0844
  2022-05-26)' '-Zcrate-attr=doc(html_root_url="https://doc.rust-lang.org/nightly/")'` (exit status: 1)
