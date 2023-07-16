plain
     |
note: previously defined here
    --> /checkout/library/core/src/macros/mod.rs:1310:5
     |
1310 | /     macro_rules! cfg {
1311 | |         ($($cfg:tt)*) => {
1313 | |         };
1314 | |     }
     | |_____^


error: aborting due to previous error

For more information about this error, try `rustc --explain E0773`.
error: doctest failed, to rerun pass `-p core --doc`
Caused by:
Caused by:
  process didn't exit successfully: `/checkout/obj/build/bootstrap/debug/rustdoc --edition=2021 --crate-type lib --crate-name core --test /checkout/library/core/src/lib.rs --target x86_64-unknown-linux-gnu -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/deps -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/release/deps -L native=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/build/compiler_builtins-b52adc3381d2c5df/out --test-args --quiet --extern core=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/deps/libcore-fa992565d2130c71.rlib --extern rand=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/deps/librand-5f5a08803cfd7db0.rlib --extern rand_xorshift=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/deps/librand_xorshift-1f0433028f46d505.rlib -C embed-bitcode=no -Zunstable-options --check-cfg 'names()' --check-cfg 'values()' -Csymbol-mangling-version=legacy -Zunstable-options -Zunstable-options '--check-cfg=values(bootstrap)' '--check-cfg=values(stdarch_intel_sde)' '--check-cfg=values(no_fp_fmt_parse)' '--check-cfg=values(no_global_oom_handling)' '--check-cfg=values(no_rc)' '--check-cfg=values(no_sync)' '--check-cfg=values(freebsd12)' '--check-cfg=values(backtrace_in_libstd)' '--check-cfg=values(target_env,"libnx")' '--check-cfg=values(target_os,"watchos")' '--check-cfg=values(target_arch,"asmjs","spirv","nvptx","nvptx64","le32","xtensa")' '--check-cfg=values(dont_compile_me)' '--check-cfg=values(rustix_use_libc)' -Dwarnings '-Wrustdoc::invalid_codeblock_attributes' --crate-version '1.68.0-nightly
  (f49eaf41d
  2023-01-02)' '-Zcrate-attr=doc(html_root_url="https://doc.rust-lang.org/nightly/")' --error-format human` (exit status: 1)
