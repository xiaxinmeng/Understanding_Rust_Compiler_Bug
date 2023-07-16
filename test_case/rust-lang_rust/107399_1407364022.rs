plain
[TIMING] compile::Assemble { target_compiler: Compiler { stage: 2, host: x86_64-unknown-linux-gnu } } -- 0.015
[TIMING] compile::StartupObjects { compiler: Compiler { stage: 2, host: x86_64-unknown-linux-gnu }, target: wasm32-unknown-unknown } -- 0.000
[TIMING] builder::Builder::sysroot_libdir::Libdir { compiler: Compiler { stage: 2, host: x86_64-unknown-linux-gnu }, target: wasm32-unknown-unknown } -- 0.000
Building stage2 library artifacts (x86_64-unknown-linux-gnu -> wasm32-unknown-unknown)
error: output of --print=file-names has changed in the compiler, cannot parse
command was: `/checkout/obj/build/bootstrap/debug/rustc - --crate-name ___ --print=file-names -Csymbol-mangling-version=legacy -Zunstable-options -Zunstable-options '--check-cfg=values(bootstrap)' '--check-cfg=values(stdarch_intel_sde)' '--check-cfg=values(no_fp_fmt_parse)' '--check-cfg=values(no_global_oom_handling)' '--check-cfg=values(no_rc)' '--check-cfg=values(no_sync)' '--check-cfg=values(freebsd12)' '--check-cfg=values(backtrace_in_libstd)' '--check-cfg=values(target_env,"libnx")' '--check-cfg=values(target_os,"watchos")' '--check-cfg=values(target_arch,"asmjs","spirv","nvptx","nvptx64","le32","xtensa")' '--check-cfg=values(dont_compile_me)' '--check-cfg=values(rustix_use_libc)' -Zmacro-backtrace -Zunstable-options -Csplit-debuginfo=off -Cprefer-dynamic -Cembed-bitcode=yes '-Zcrate-attr=doc(html_root_url="https://doc.rust-lang.org/nightly/")' --target wasm32-unknown-unknown --crate-type bin --crate-type rlib --crate-type dylib --crate-type cdylib --crate-type staticlib --crate-type proc-macro --print=sysroot --print=cfg`
--- stdout
___.wasm
___.wasm
lib___.rlib
___.wasm
lib___.a
/checkout/obj/build/x86_64-unknown-linux-gnu/stage2
debug_assertions
panic="abort"
panic="abort"
proc_macro
target_abi=""
target_arch="wasm32"
target_endian="little"
target_env=""
target_family="wasm"
target_feature="llvm14-builtins-abi"
target_has_atomic="16"
target_has_atomic="32"
target_has_atomic="64"
target_has_atomic="8"
target_has_atomic="8"
target_has_atomic="ptr"
target_has_atomic_equal_alignment="16"
target_has_atomic_equal_alignment="32"
target_has_atomic_equal_alignment="64"
target_has_atomic_equal_alignment="8"
target_has_atomic_equal_alignment="ptr"
target_has_atomic_load_store="16"
target_has_atomic_load_store="32"
target_has_atomic_load_store="64"
target_has_atomic_load_store="8"
target_has_atomic_load_store="8"
target_has_atomic_load_store="ptr"
target_os="unknown"
target_pointer_width="32"
target_thread_local
target_vendor="unknown"
--- stderr
warning: dropping unsupported crate type `dylib` for target `wasm32-unknown-unknown`

warning: 1 warning emitted
warning: 1 warning emitted

[RUSTC-TIMING] ___ test:false 0.022
