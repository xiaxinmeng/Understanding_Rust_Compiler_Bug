plain
Building stage1 library artifacts (x86_64-unknown-linux-gnu) 
error: failed to run `rustc` to learn about target-specific information

Caused by:
  process didn't exit successfully: `/checkout/obj/build/bootstrap/debug/rustc - --crate-name ___ --print=file-names -Csymbol-mangling-version=legacy -Zunstable-options -Zunstable-options '--check-cfg=values(bootstrap)' '--check-cfg=values(stdarch_intel_sde)' '--check-cfg=values(no_fp_fmt_parse)' '--check-cfg=values(no_global_oom_handling)' '--check-cfg=values(no_rc)' '--check-cfg=values(no_sync)' '--check-cfg=values(freebsd12)' '--check-cfg=values(backtrace_in_libstd)' '--check-cfg=values(target_env,"libnx")' '--check-cfg=values(target_arch,"asmjs","spirv","nvptx","xtensa")' -Zmacro-backtrace -Clink-args=-Wl,-z,origin '-Clink-args=-Wl,-rpath,$ORIGIN/../lib' -Csplit-debuginfo=off -Cprefer-dynamic -Cembed-bitcode=yes '-Zcrate-attr=doc(html_root_url="https://doc.rust-lang.org/nightly/")' --target x86_64-unknown-linux-gnu --crate-type bin --crate-type rlib --crate-type dylib --crate-type cdylib --crate-type staticlib --crate-type proc-macro --print=sysroot --print=split-debuginfo --print=crate-name --print=cfg` (exit status: 254)
  --- stderr
  /checkout/obj/build/x86_64-unknown-linux-gnu/stage1/lib/librustc_driver-fdca23ff32bf7124.so(+0x97cff3)[0x7f21a1522ff3]
  /lib/x86_64-linux-gnu/libc.so.6(+0x42520)[0x7f21a07fa520]
  /checkout/obj/build/x86_64-unknown-linux-gnu/stage1/lib/libLLVM-16-rust-1.70.0-nightly.so(+0x7262e06)[0x7f219f062e06]
  /checkout/obj/build/x86_64-unknown-linux-gnu/stage1/lib/librustc_driver-fdca23ff32bf7124.so(+0xd10d87)[0x7f21a18b6d87]
  /checkout/obj/build/x86_64-unknown-linux-gnu/stage1/lib/librustc_driver-fdca23ff32bf7124.so(+0xbc960c)[0x7f21a176f60c]
  /checkout/obj/build/x86_64-unknown-linux-gnu/stage1/lib/librustc_driver-fdca23ff32bf7124.so(+0xbc8744)[0x7f21a176e744]
  /checkout/obj/build/x86_64-unknown-linux-gnu/stage1/lib/librustc_driver-fdca23ff32bf7124.so(_RNvXs5_Cs7DTEPoQIU0M_18rustc_codegen_llvmNtB5_18LlvmCodegenBackendNtNtNtCsiydnvZ3aRh1_17rustc_codegen_ssa6traits7backend14CodegenBackend15target_features+0x1b)[0x7f21a17ce77b]
  /checkout/obj/build/x86_64-unknown-linux-gnu/stage1/lib/librustc_driver-fdca23ff32bf7124.so(_RNvNtCsbMR7lNZiyEF_15rustc_interface4util17add_configuration+0x37)[0x7f21a1610837]
  /checkout/obj/build/x86_64-unknown-linux-gnu/stage1/lib/librustc_driver-fdca23ff32bf7124.so(_RNvNtCsbMR7lNZiyEF_15rustc_interface4util14create_session+0x89d)[0x7f21a16112dd]
Build completed unsuccessfully in 0:03:09
  /checkout/obj/build/x86_64-unknown-linux-gnu/stage1/lib/librustc_driver-fdca23ff32bf7124.so(+0x987540)[0x7f21a152d540]
  /checkout/obj/build/x86_64-unknown-linux-gnu/stage1/lib/librustc_driver-fdca23ff32bf7124.so(+0x9822c0)[0x7f21a15282c0]
  /checkout/obj/build/x86_64-unknown-linux-gnu/stage1/lib/librustc_driver-fdca23ff32bf7124.so(+0x983e03)[0x7f21a1529e03]
  /checkout/obj/build/x86_64-unknown-linux-gnu/stage1/lib/libstd-2268eb07c4da6a07.so(rust_metadata_std_e292dd3bffb96032+0xca46e)[0x7f21a0aae46e]
  /lib/x86_64-linux-gnu/libc.so.6(+0x94b43)[0x7f21a084cb43]
  /lib/x86_64-linux-gnu/libc.so.6(+0x126a00)[0x7f21a08dea00]
  [RUSTC-TIMING] ___ test:false 0.182
  rustc exited with signal: 11 (SIGSEGV) (core dumped)
make: *** [Makefile:44: check-aux] Error 1
