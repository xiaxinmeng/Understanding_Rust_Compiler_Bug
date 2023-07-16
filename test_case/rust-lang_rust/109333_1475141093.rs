plain
Prepare all required actions
Getting action download info
Download action repository 'actions/checkout@v3' (SHA:24cb9080177205b6e8c946b17badbe402adc938f)
Download action repository 'rust-lang/simpleinfra@master' (SHA:3fb2b44a4eaebb9ed8086446bde46c27199ef5ed)
Complete job name: PR (x86_64-gnu-tools, false, ubuntu-20.04-xl)
git config --global core.autocrlf false
shell: /usr/bin/bash --noprofile --norc -e -o pipefail {0}
env:
  CI_JOB_NAME: x86_64-gnu-tools
---
Building stage1 library artifacts (x86_64-unknown-linux-gnu) 
error: failed to run `rustc` to learn about target-specific information

Caused by:
  process didn't exit successfully: `/checkout/obj/build/bootstrap/debug/rustc - --crate-name ___ --print=file-names -Csymbol-mangling-version=legacy -Zunstable-options -Zunstable-options '--check-cfg=values(bootstrap)' '--check-cfg=values(stdarch_intel_sde)' '--check-cfg=values(no_fp_fmt_parse)' '--check-cfg=values(no_global_oom_handling)' '--check-cfg=values(no_rc)' '--check-cfg=values(no_sync)' '--check-cfg=values(freebsd12)' '--check-cfg=values(backtrace_in_libstd)' '--check-cfg=values(target_env,"libnx")' '--check-cfg=values(target_arch,"asmjs","spirv","nvptx","xtensa")' -Zmacro-backtrace -Clink-args=-Wl,-z,origin '-Clink-args=-Wl,-rpath,$ORIGIN/../lib' -Csplit-debuginfo=off -Cprefer-dynamic -Cembed-bitcode=yes '-Zcrate-attr=doc(html_root_url="https://doc.rust-lang.org/nightly/")' --target x86_64-unknown-linux-gnu --crate-type bin --crate-type rlib --crate-type dylib --crate-type cdylib --crate-type staticlib --crate-type proc-macro --print=sysroot --print=split-debuginfo --print=crate-name --print=cfg` (exit status: 254)
  --- stderr
  /checkout/obj/build/x86_64-unknown-linux-gnu/stage1/lib/librustc_driver-fdca23ff32bf7124.so(+0x979813)[0x7fcfadd87813]
  /lib/x86_64-linux-gnu/libc.so.6(+0x42520)[0x7fcfad062520]
  /checkout/obj/build/x86_64-unknown-linux-gnu/stage1/lib/libLLVM-16-rust-1.70.0-nightly.so(+0x7262e06)[0x7fcfab862e06]
  /checkout/obj/build/x86_64-unknown-linux-gnu/stage1/lib/librustc_driver-fdca23ff32bf7124.so(+0xd100e7)[0x7fcfae11e0e7]
  /checkout/obj/build/x86_64-unknown-linux-gnu/stage1/lib/librustc_driver-fdca23ff32bf7124.so(+0xbbb15c)[0x7fcfadfc915c]
  /checkout/obj/build/x86_64-unknown-linux-gnu/stage1/lib/librustc_driver-fdca23ff32bf7124.so(+0xbba204)[0x7fcfadfc8204]
  /checkout/obj/build/x86_64-unknown-linux-gnu/stage1/lib/librustc_driver-fdca23ff32bf7124.so(_RNvXs5_Cs7DTEPoQIU0M_18rustc_codegen_llvmNtB5_18LlvmCodegenBackendNtNtNtCsiydnvZ3aRh1_17rustc_codegen_ssa6traits7backend14CodegenBackend15target_features+0x1b)[0x7fcfae02882b]
  /checkout/obj/build/x86_64-unknown-linux-gnu/stage1/lib/librustc_driver-fdca23ff32bf7124.so(_RNvNtCsbMR7lNZiyEF_15rustc_interface4util17add_configuration+0x37)[0x7fcfade8d8e7]
  /checkout/obj/build/x86_64-unknown-linux-gnu/stage1/lib/librustc_driver-fdca23ff32bf7124.so(_RNvNtCsbMR7lNZiyEF_15rustc_interface4util14create_session+0x89d)[0x7fcfade8e38d]
  /checkout/obj/build/x86_64-unknown-linux-gnu/stage1/lib/librustc_driver-fdca23ff32bf7124.so(+0x983d60)[0x7fcfadd91d60]
  /checkout/obj/build/x86_64-unknown-linux-gnu/stage1/lib/librustc_driver-fdca23ff32bf7124.so(+0x97eae0)[0x7fcfadd8cae0]
  /checkout/obj/build/x86_64-unknown-linux-gnu/stage1/lib/librustc_driver-fdca23ff32bf7124.so(+0x980623)[0x7fcfadd8e623]
  /checkout/obj/build/x86_64-unknown-linux-gnu/stage1/lib/libstd-2268eb07c4da6a07.so(rust_metadata_std_e292dd3bffb96032+0xca46e)[0x7fcfad31646e]
  /lib/x86_64-linux-gnu/libc.so.6(+0x94b43)[0x7fcfad0b4b43]
  /lib/x86_64-linux-gnu/libc.so.6(+0x126a00)[0x7fcfad146a00]
  rustc exited with signal: 11 (SIGSEGV) (core dumped)
cat: /tmp/toolstate/toolstates.json: No such file or directory
