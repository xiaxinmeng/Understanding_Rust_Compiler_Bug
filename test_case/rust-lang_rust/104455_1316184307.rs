plain
 Documenting alloc v0.0.0 (/checkout/library/alloc)
error: unresolved link to `retain`
    --> library/alloc/src/collections/btree/map.rs:1356:15
     |
1356 |     /// Use [`retain()`] with a negated predicate if you do not need the returned iterator.
     |               ^^^^^^^^ no item named `retain` in scope
     |
     = help: to escape `[` and `]` characters, add '\' before them like `\[` or `\]`
     = note: `-D rustdoc::broken-intra-doc-links` implied by `-D warnings`
error: unresolved link to `retain`
    --> library/alloc/src/collections/btree/set.rs:1096:15
     |
     |
1096 |     /// Use [`retain()`] with a negated predicate if you do not need the returned iterator.
     |               ^^^^^^^^ no item named `retain` in scope
     |
     = help: to escape `[` and `]` characters, add '\' before them like `\[` or `\]`
error: unresolved link to `retain`
   --> library/alloc/src/collections/linked_list.rs:972:15
    |
    |
972 |     /// Use [`retain()`] with a negated predicate if you do not need the returned iterator.
    |               ^^^^^^^^ no item named `retain` in scope
    |
    = help: to escape `[` and `]` characters, add '\' before them like `\[` or `\]`
error: unresolved link to `retain`
    --> library/alloc/src/vec/mod.rs:2925:15
     |
     |
2925 |     /// Use [`retain()`] with a negated predicate if you do not need the returned iterator.
     |               ^^^^^^^^ no item named `retain` in scope
     |
     = help: to escape `[` and `]` characters, add '\' before them like `\[` or `\]`
error: could not document `alloc`

Caused by:
Caused by:
  process didn't exit successfully: `/checkout/obj/build/bootstrap/debug/rustdoc --edition=2021 --crate-type lib --crate-name alloc library/alloc/src/lib.rs --target x86_64-unknown-linux-gnu -o /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-unknown-linux-gnu/doc --cfg 'feature="compiler-builtins-c"' -Zunstable-options --check-cfg 'names()' --check-cfg 'values()' --error-format=json --json=diagnostic-rendered-ansi,artifacts,future-incompat -Z unstable-options --resource-suffix 1.67.0 --markdown-css rust.css --markdown-no-toc --index-page /checkout/src/doc/index.md -C metadata=6bad7816088fa58e -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-unknown-linux-gnu/release/deps -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/release/deps --extern compiler_builtins=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-unknown-linux-gnu/release/deps/libcompiler_builtins-1df8740800b4bc27.rmeta --extern core=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-unknown-linux-gnu/release/deps/libcore-b98de7aec7c46ce5.rmeta -Csymbol-mangling-version=legacy -Zunstable-options -Zunstable-options '--check-cfg=values(bootstrap)' '--check-cfg=values(stdarch_intel_sde)' '--check-cfg=values(no_fp_fmt_parse)' '--check-cfg=values(no_global_oom_handling)' '--check-cfg=values(no_rc)' '--check-cfg=values(no_sync)' '--check-cfg=values(freebsd12)' '--check-cfg=values(backtrace_in_libstd)' '--check-cfg=values(target_env,"libnx")' '--check-cfg=values(target_os,"watchos")' '--check-cfg=values(target_arch,"asmjs","spirv","nvptx","nvptx64","le32","xtensa")' '--check-cfg=values(dont_compile_me)' '--check-cfg=values(rustix_use_libc)' -Dwarnings '-Wrustdoc::invalid_codeblock_attributes' --crate-version '1.67.0-nightly
  (ebb7dd784
  2022-11-15)' '-Zcrate-attr=doc(html_root_url="https://doc.rust-lang.org/nightly/")'` (exit status: 1)
