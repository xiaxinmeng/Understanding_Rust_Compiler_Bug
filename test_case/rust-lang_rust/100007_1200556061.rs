
Building stage1 std artifacts (x86_64-pc-windows-msvc -> x86_64-pc-windows-msvc)
   Compiling cc v1.0.69
   Compiling core v0.0.0 (R:\Rust\rust\library\core)
   Compiling libc v0.2.126
   Compiling memchr v2.5.0
   Compiling std v0.0.0 (R:\Rust\rust\library\std)
error: could not compile `core`

Caused by:
  process didn't exit successfully: `R:\Rust\rust\build\bootstrap\debug\rustc --crate-name core --edition=2021 library\core\src\lib.rs --error-format=json --json=diagnostic-rendered-ansi,artifacts,future-incompat --crate-type lib --emit=dep-info,metadata,link -C opt-level=3 -C embed-bitcode=no -C debuginfo=0 -Zunstable-options --check-cfg names() --check-cfg values() -C metadata=7a8ae34f67ccc69f -C extra-filename=-7a8ae34f67ccc69f --out-dir R:\Rust\rust\build\x86_64-pc-windows-msvc\stage1-std\x86_64-pc-windows-msvc\release\deps --target x86_64-pc-windows-msvc -C incremental=R:\Rust\rust\build\x86_64-pc-windows-msvc\stage1-std\x86_64-pc-windows-msvc\release\incremental -L dependency=R:\Rust\rust\build\x86_64-pc-windows-msvc\stage1-std\x86_64-pc-windows-msvc\release\deps -L dependency=R:\Rust\rust\build\x86_64-pc-windows-msvc\stage1-std\release\deps -Csymbol-mangling-version=legacy -Zunstable-options -Zunstable-options --check-cfg=values(bootstrap) --check-cfg=values(stdarch_intel_sde) --check-cfg=values(no_fp_fmt_parse) --check-cfg=values(no_global_oom_handling) --check-cfg=values(freebsd12) --check-cfg=values(backtrace_in_libstd) "--check-cfg=values(target_env,\"libnx\")" "--check-cfg=values(target_os,\"watchos\")" "--check-cfg=values(target_arch,\"asmjs\",\"spirv\",\"nvptx\",\"nvptx64\",\"le32\",\"xtensa\")" --check-cfg=values(dont_compile_me) -Zmacro-backtrace -Ctarget-feature=+crt-static -Cprefer-dynamic -Cllvm-args=-import-instr-limit=10 -Cembed-bitcode=yes "-Zcrate-attr=doc(html_root_url=\"https://doc.rust-lang.org/nightly/\")" -Z binary-dep-depinfo` (exit code: 0xc0000005, STATUS_ACCESS_VIOLATION)
warning: build failed, waiting for other jobs to finish...
Build completed unsuccessfully in 0:05:38
