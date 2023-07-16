plain
 Documenting alloc v0.0.0 (/checkout/library/alloc)
error: unresolved link to `from_iter`
    --> library/alloc/src/collections/vec_deque/mod.rs:2875:16
     |
2875 |     /// Like [`<VecDeque::<T>>::from_iter()`], but coallocation-aware.
     |
     |
     = help: to escape `[` and `]` characters, add '\' before them like `\[` or `\]`
     = note: `-D rustdoc::broken-intra-doc-links` implied by `-D warnings`
error: unresolved link to `into_vec`
   --> library/alloc/src/slice.rs:647:41
    |
    |
647 |     /// Coallocation-aware version of [`into_vec<A>()`].
    |
    |
    = help: to escape `[` and `]` characters, add '\' before them like `\[` or `\]`
error: unresolved link to `into_vec`
   --> library/alloc/src/slice.rs:648:41
    |
    |
648 |     /// Coallocation-aware version of [`into_vec::<A>()`].
    |
    |
    = help: to escape `[` and `]` characters, add '\' before them like `\[` or `\]`
error: could not document `alloc`

Caused by:
Caused by:
  process didn't exit successfully: `/checkout/obj/build/bootstrap/debug/rustdoc --edition=2021 --crate-type lib --crate-name alloc library/alloc/src/lib.rs --target x86_64-unknown-linux-gnu -o /checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/doc/x86_64-unknown-linux-gnu/doc --cfg 'feature="compiler-builtins-c"' -Zunstable-options --check-cfg 'names()' --check-cfg 'values()' --error-format=json --json=diagnostic-rendered-ansi,artifacts,future-incompat -Z unstable-options --resource-suffix 1.70.0 --markdown-css rust.css --markdown-no-toc --index-page /checkout/src/doc/index.md -C metadata=7985f56da7779bd9 -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/doc/x86_64-unknown-linux-gnu/release/deps -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/doc/release/deps --extern compiler_builtins=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/doc/x86_64-unknown-linux-gnu/release/deps/libcompiler_builtins-bf5b44bc667e6de6.rmeta --extern core=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/doc/x86_64-unknown-linux-gnu/release/deps/libcore-77221b083c66e738.rmeta --cfg=bootstrap -Csymbol-mangling-version=legacy -Zunstable-options -Zunstable-options '--check-cfg=values(bootstrap)' '--check-cfg=values(stdarch_intel_sde)' '--check-cfg=values(no_fp_fmt_parse)' '--check-cfg=values(no_global_oom_handling)' '--check-cfg=values(no_rc)' '--check-cfg=values(no_sync)' '--check-cfg=values(freebsd12)' '--check-cfg=values(backtrace_in_libstd)' '--check-cfg=values(target_env,"libnx")' '--check-cfg=values(target_arch,"asmjs","spirv","nvptx","xtensa")' --document-private-items --document-hidden-items -Dwarnings '-Wrustdoc::invalid_codeblock_attributes' --crate-version '1.70.0-nightly
  (29395d948
  2023-03-15)' '-Zcrate-attr=doc(html_root_url="https://doc.rust-lang.org/nightly/")'` (exit status: 1)
