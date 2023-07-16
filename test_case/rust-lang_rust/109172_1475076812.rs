plain
Building stage1 library artifacts (x86_64-unknown-linux-gnu) 
error: failed to run `rustc` to learn about target-specific information

Caused by:
  process didn't exit successfully: `/checkout/obj/build/bootstrap/debug/rustc - --crate-name ___ --print=file-names -Csymbol-mangling-version=legacy -Zunstable-options -Zunstable-options '--check-cfg=values(bootstrap)' '--check-cfg=values(stdarch_intel_sde)' '--check-cfg=values(no_fp_fmt_parse)' '--check-cfg=values(no_global_oom_handling)' '--check-cfg=values(no_rc)' '--check-cfg=values(no_sync)' '--check-cfg=values(freebsd12)' '--check-cfg=values(backtrace_in_libstd)' '--check-cfg=values(target_env,"libnx")' '--check-cfg=values(target_arch,"asmjs","spirv","nvptx","xtensa")' -Zmacro-backtrace -Clink-args=-Wl,-z,origin '-Clink-args=-Wl,-rpath,$ORIGIN/../lib' -Csplit-debuginfo=off -Cprefer-dynamic -Cembed-bitcode=yes '-Zcrate-attr=doc(html_root_url="https://doc.rust-lang.org/nightly/")' --target x86_64-unknown-linux-gnu --crate-type bin --crate-type rlib --crate-type dylib --crate-type cdylib --crate-type staticlib --crate-type proc-macro --print=sysroot --print=split-debuginfo --print=crate-name --print=cfg` (exit status: 254)
  --- stderr
  /checkout/obj/build/x86_64-unknown-linux-gnu/stage1/lib/librustc_driver-fdca23ff32bf7124.so(+0x97cff3)[0x7f6d6ef3aff3]
  /lib/x86_64-linux-gnu/libc.so.6(+0x42520)[0x7f6d6e212520]
  /checkout/obj/build/x86_64-unknown-linux-gnu/stage1/lib/libLLVM-16-rust-1.70.0-nightly.so(+0x7262e06)[0x7f6d6ca62e06]
  /checkout/obj/build/x86_64-unknown-linux-gnu/stage1/lib/librustc_driver-fdca23ff32bf7124.so(+0xd10d87)[0x7f6d6f2ced87]
  /checkout/obj/build/x86_64-unknown-linux-gnu/stage1/lib/librustc_driver-fdca23ff32bf7124.so(+0xbc960c)[0x7f6d6f18760c]
  /checkout/obj/build/x86_64-unknown-linux-gnu/stage1/lib/librustc_driver-fdca23ff32bf7124.so(+0xbc8744)[0x7f6d6f186744]
  /checkout/obj/build/x86_64-unknown-linux-gnu/stage1/lib/librustc_driver-fdca23ff32bf7124.so(_RNvXs5_Cs7DTEPoQIU0M_18rustc_codegen_llvmNtB5_18LlvmCodegenBackendNtNtNtCsiydnvZ3aRh1_17rustc_codegen_ssa6traits7backend14CodegenBackend15target_features+0x1b)[0x7f6d6f1e677b]
  /checkout/obj/build/x86_64-unknown-linux-gnu/stage1/lib/librustc_driver-fdca23ff32bf7124.so(_RNvNtCsbMR7lNZiyEF_15rustc_interface4util17add_configuration+0x37)[0x7f6d6f028837]
Build completed unsuccessfully in 0:03:17
  /checkout/obj/build/x86_64-unknown-linux-gnu/stage1/lib/librustc_driver-fdca23ff32bf7124.so(_RNvNtCsbMR7lNZiyEF_15rustc_interface4util14create_session+0x89d)[0x7f6d6f0292dd]
  /checkout/obj/build/x86_64-unknown-linux-gnu/stage1/lib/librustc_driver-fdca23ff32bf7124.so(+0x987540)[0x7f6d6ef45540]
  /checkout/obj/build/x86_64-unknown-linux-gnu/stage1/lib/librustc_driver-fdca23ff32bf7124.so(+0x9822c0)[0x7f6d6ef402c0]
  /checkout/obj/build/x86_64-unknown-linux-gnu/stage1/lib/librustc_driver-fdca23ff32bf7124.so(+0x983e03)[0x7f6d6ef41e03]
  /checkout/obj/build/x86_64-unknown-linux-gnu/stage1/lib/libstd-2268eb07c4da6a07.so(rust_metadata_std_e292dd3bffb96032+0xca46e)[0x7f6d6e4c646e]
  /lib/x86_64-linux-gnu/libc.so.6(+0x94b43)[0x7f6d6e264b43]
  /lib/x86_64-linux-gnu/libc.so.6(+0x126a00)[0x7f6d6e2f6a00]
  [RUSTC-TIMING] ___ test:false 0.246
  rustc exited with signal: 11 (SIGSEGV) (core dumped)
make: *** [Makefile:44: check-aux] Error 1
