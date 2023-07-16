plain
Prepare all required actions
Getting action download info
Download action repository 'actions/checkout@v3' (SHA:ac593985615ec2ede58e132d2e21d2b1cbd6127c)
Download action repository 'rust-lang/simpleinfra@master' (SHA:ece894d15649f0f9d7884f915dc821f00bd0418b)
Complete job name: PR (x86_64-gnu-llvm-13, false, ubuntu-20.04-xl)
git config --global core.autocrlf false
shell: /usr/bin/bash --noprofile --norc -e -o pipefail {0}
env:
  CI_JOB_NAME: x86_64-gnu-llvm-13
---
   Compiling object v0.29.0
   Compiling hashbrown v0.12.3
   Compiling std_detect v0.1.5 (/checkout/library/stdarch/crates/std_detect)
   Compiling miniz_oxide v0.5.3
Invalid InsertValueInst operands!
  %17 = insertvalue { {}*, [3 x i64]* } %16, i64* %13, 1
LLVM ERROR: Broken module found, compilation aborted!
error: could not compile `panic_unwind`
warning: build failed, waiting for other jobs to finish...
/checkout/obj/build/x86_64-unknown-linux-gnu/stage1/lib/librustc_driver-557174ce2fd6a0af.so(+0x8ff2f3)[0x7f24877fc2f3]
/lib/x86_64-linux-gnu/libc.so.6(+0x42520)[0x7f2486b47520]
/lib/x86_64-linux-gnu/libLLVM-13.so.1(_ZN4llvm11PointerType3getEPNS_4TypeEj+0x20)[0x7f2481ac7bd0]
/lib/x86_64-linux-gnu/libLLVM-13.so.1(+0xe400ac)[0x7f24819a10ac]
/lib/x86_64-linux-gnu/libLLVM-13.so.1(+0x1960c29)[0x7f24824c1c29]
/lib/x86_64-linux-gnu/libLLVM-13.so.1(+0x196025e)[0x7f24824c125e]
/lib/x86_64-linux-gnu/libLLVM-13.so.1(+0x1964e21)[0x7f24824c5e21]
/lib/x86_64-linux-gnu/libLLVM-13.so.1(+0x1965fbc)[0x7f24824c6fbc]
/lib/x86_64-linux-gnu/libLLVM-13.so.1(_ZN4llvm15InstCombinePass3runERNS_8FunctionERNS_15AnalysisManagerIS1_JEEE+0x230)[0x7f24824c6b40]
/lib/x86_64-linux-gnu/libLLVM-13.so.1(+0x349f7fd)[0x7f24840007fd]
/lib/x86_64-linux-gnu/libLLVM-13.so.1(_ZN4llvm11PassManagerINS_8FunctionENS_15AnalysisManagerIS1_JEEEJEE3runERS1_RS3_+0x19e)[0x7f2481aaf61e]
/lib/x86_64-linux-gnu/libLLVM-13.so.1(+0x284937d)[0x7f24833aa37d]
/lib/x86_64-linux-gnu/libLLVM-13.so.1(_ZN4llvm27ModuleToFunctionPassAdaptor3runERNS_6ModuleERNS_15AnalysisManagerIS1_JEEE+0x1da)[0x7f2481ab31fa]
/lib/x86_64-linux-gnu/libLLVM-13.so.1(+0x28491bd)[0x7f24833aa1bd]
/lib/x86_64-linux-gnu/libLLVM-13.so.1(_ZN4llvm11PassManagerINS_6ModuleENS_15AnalysisManagerIS1_JEEEJEE3runERS1_RS3_+0x19e)[0x7f2481aae35e]
/checkout/obj/build/x86_64-unknown-linux-gnu/stage1/lib/librustc_driver-557174ce2fd6a0af.so(+0xcaa3d5)[0x7f2487ba73d5]
/checkout/obj/build/x86_64-unknown-linux-gnu/stage1/lib/librustc_driver-557174ce2fd6a0af.so(+0xb5c68a)[0x7f2487a5968a]
/checkout/obj/build/x86_64-unknown-linux-gnu/stage1/lib/librustc_driver-557174ce2fd6a0af.so(+0xb5cc25)[0x7f2487a59c25]
/checkout/obj/build/x86_64-unknown-linux-gnu/stage1/lib/librustc_driver-557174ce2fd6a0af.so(+0xb78a79)[0x7f2487a75a79]
/checkout/obj/build/x86_64-unknown-linux-gnu/stage1/lib/librustc_driver-557174ce2fd6a0af.so(+0xc67316)[0x7f2487b64316]
/checkout/obj/build/x86_64-unknown-linux-gnu/stage1/lib/librustc_driver-557174ce2fd6a0af.so(+0xbe4aba)[0x7f2487ae1aba]
/checkout/obj/build/x86_64-unknown-linux-gnu/stage1/lib/librustc_driver-557174ce2fd6a0af.so(+0xc3e2cf)[0x7f2487b3b2cf]
/checkout/obj/build/x86_64-unknown-linux-gnu/stage1/lib/libstd-2f96cead2d2eaf8f.so(rust_metadata_std_d2c56ec1bbb131e1+0xd440e)[0x7f2486e0640e]
/lib/x86_64-linux-gnu/libc.so.6(+0x94b43)[0x7f2486b99b43]
/lib/x86_64-linux-gnu/libc.so.6(+0x126a00)[0x7f2486c2ba00]
rustc exited with signal: 11 (SIGSEGV) (core dumped)
error: could not compile `gimli`
Caused by:
Caused by:
  process didn't exit successfully: `/checkout/obj/build/bootstrap/debug/rustc --crate-name gimli --edition=2018 /cargo/registry/src/github.com-1ecc6299db9ec823/gimli-0.26.2/src/lib.rs --error-format=json --json=diagnostic-rendered-ansi,artifacts,future-incompat --crate-type lib --emit=dep-info,metadata,link -C opt-level=3 -C embed-bitcode=no -C codegen-units=1 -C debuginfo=0 -C debug-assertions=on --cfg 'feature="alloc"' --cfg 'feature="compiler_builtins"' --cfg 'feature="core"' --cfg 'feature="read"' --cfg 'feature="read-core"' --cfg 'feature="rustc-dep-of-std"' -Zunstable-options --check-cfg 'names()' --check-cfg 'values()' -C metadata=ba2c5ffa1e4486e0 -C extra-filename=-ba2c5ffa1e4486e0 --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/deps --target x86_64-unknown-linux-gnu -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/deps -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/release/deps --extern compiler_builtins=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/deps/libcompiler_builtins-4f27673eda043c10.rmeta --extern alloc=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/deps/librustc_std_workspace_alloc-83fa51bdee139e48.rmeta --extern core=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/deps/librustc_std_workspace_core-a842d6fd47d5c6b1.rmeta --cap-lints allow -Csymbol-mangling-version=legacy -Zunstable-options -Zunstable-options '--check-cfg=values(bootstrap)' '--check-cfg=values(stdarch_intel_sde)' '--check-cfg=values(no_fp_fmt_parse)' '--check-cfg=values(no_global_oom_handling)' '--check-cfg=values(no_rc)' '--check-cfg=values(no_sync)' '--check-cfg=values(freebsd12)' '--check-cfg=values(backtrace_in_libstd)' '--check-cfg=values(target_env,"libnx")' '--check-cfg=values(target_os,"watchos")' '--check-cfg=values(target_arch,"asmjs","spirv","nvptx","nvptx64","le32","xtensa")' '--check-cfg=values(dont_compile_me)' '--check-cfg=values(rustix_use_libc)' -Zmacro-backtrace -Clink-args=-Wl,-z,origin '-Clink-args=-Wl,-rpath,$ORIGIN/../lib' -Csplit-debuginfo=off -Cprefer-dynamic -Cllvm-args=-import-instr-limit=10 -Cembed-bitcode=yes '-Zcrate-attr=doc(html_root_url="https://doc.rust-lang.org/nightly/")' -Z binary-dep-depinfo -L native=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/build/compiler_builtins-23df797a2ec042c2/out` (exit status: 254)
