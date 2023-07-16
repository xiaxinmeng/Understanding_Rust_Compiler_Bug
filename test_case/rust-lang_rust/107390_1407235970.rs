plain
[TIMING] compile::Assemble { target_compiler: Compiler { stage: 2, host: x86_64-unknown-linux-gnu } } -- 0.004
[TIMING] compile::StartupObjects { compiler: Compiler { stage: 2, host: x86_64-unknown-linux-gnu }, target: thumbv6m-none-eabi } -- 0.000
[TIMING] builder::Builder::sysroot_libdir::Libdir { compiler: Compiler { stage: 2, host: x86_64-unknown-linux-gnu }, target: thumbv6m-none-eabi } -- 0.000
Building stage2 library artifacts (x86_64-unknown-linux-gnu -> thumbv6m-none-eabi)
error: output of --print=file-names has changed in the compiler, cannot parse
command was: `/checkout/obj/build/bootstrap/debug/rustc - --crate-name ___ --print=file-names -Csymbol-mangling-version=legacy -Zunstable-options -Zunstable-options '--check-cfg=values(bootstrap)' '--check-cfg=values(stdarch_intel_sde)' '--check-cfg=values(no_fp_fmt_parse)' '--check-cfg=values(no_global_oom_handling)' '--check-cfg=values(no_rc)' '--check-cfg=values(no_sync)' '--check-cfg=values(freebsd12)' '--check-cfg=values(backtrace_in_libstd)' '--check-cfg=values(target_env,"libnx")' '--check-cfg=values(target_os,"watchos")' '--check-cfg=values(target_arch,"asmjs","spirv","nvptx","nvptx64","le32","xtensa")' '--check-cfg=values(dont_compile_me)' '--check-cfg=values(rustix_use_libc)' -Zmacro-backtrace -Clink-args=-Wl,-z,origin '-Clink-args=-Wl,-rpath,$ORIGIN/../lib' -Zunstable-options -Csplit-debuginfo=off -Cprefer-dynamic -Cembed-bitcode=yes '-Zcrate-attr=doc(html_root_url="https://doc.rust-lang.org/nightly/")' --target thumbv6m-none-eabi --crate-type bin --crate-type rlib --crate-type dylib --crate-type cdylib --crate-type staticlib --crate-type proc-macro --print=sysroot --print=cfg`
--- stdout
___
___
lib___.rlib
lib___.a
debug_assertions
panic="abort"
proc_macro
target_abi="eabi"
target_abi="eabi"
target_arch="arm"
target_endian="little"
target_env=""
target_feature="llvm14-builtins-abi"
target_feature="mclass"
target_feature="mclass"
target_feature="thumb-mode"
target_feature="v5te"
target_feature="v6"
target_has_atomic_equal_alignment="16"
target_has_atomic_equal_alignment="32"
target_has_atomic_equal_alignment="8"
target_has_atomic_equal_alignment="ptr"
target_has_atomic_load_store="16"
target_has_atomic_load_store="32"
target_has_atomic_load_store="8"
target_has_atomic_load_store="8"
target_has_atomic_load_store="ptr"
target_os="none"
target_pointer_width="32"
target_vendor="unknown"
--- stderr
warning: dropping unsupported crate type `dylib` for target `thumbv6m-none-eabi`

warning: 1 warning emitted
warning: 1 warning emitted

[RUSTC-TIMING] ___ test:false 0.021
