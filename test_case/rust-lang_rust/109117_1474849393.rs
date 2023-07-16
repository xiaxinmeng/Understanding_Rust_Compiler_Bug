plain
Building stage1 library artifacts (x86_64-unknown-linux-gnu) 
error: failed to run `rustc` to learn about target-specific information

Caused by:
  process didn't exit successfully: `/checkout/obj/build/bootstrap/debug/rustc - --crate-name ___ --print=file-names -Csymbol-mangling-version=legacy -Zunstable-options -Zunstable-options '--check-cfg=values(bootstrap)' '--check-cfg=values(stdarch_intel_sde)' '--check-cfg=values(no_fp_fmt_parse)' '--check-cfg=values(no_global_oom_handling)' '--check-cfg=values(no_rc)' '--check-cfg=values(no_sync)' '--check-cfg=values(freebsd12)' '--check-cfg=values(backtrace_in_libstd)' '--check-cfg=values(target_env,"libnx")' '--check-cfg=values(target_arch,"asmjs","spirv","nvptx","xtensa")' -Zmacro-backtrace -Clink-args=-Wl,-z,origin '-Clink-args=-Wl,-rpath,$ORIGIN/../lib' -Clink-args=-fuse-ld=lld -Csplit-debuginfo=off -Cprefer-dynamic -Cembed-bitcode=yes '-Zcrate-attr=doc(html_root_url="https://doc.rust-lang.org/nightly/")' --target x86_64-unknown-linux-gnu --crate-type bin --crate-type rlib --crate-type dylib --crate-type cdylib --crate-type staticlib --crate-type proc-macro --print=sysroot --print=split-debuginfo --print=crate-name --print=cfg` (exit status: 254)
  --- stderr
  /checkout/obj/build/x86_64-unknown-linux-gnu/stage1/lib/librustc_driver-9f03b5a4bd4fd338.so(+0x12a24f3)[0x7ff4b90224f3]
  /lib64/libpthread.so.0(+0xf630)[0x7ff4b7767630]
  /checkout/obj/build/x86_64-unknown-linux-gnu/stage1/bin/rustc(+0x1d0bb)[0x55c21dcb90bb]
  /checkout/obj/build/x86_64-unknown-linux-gnu/stage1/lib/librustc_driver-9f03b5a4bd4fd338.so(+0x13c5a9e)[0x7ff4b9145a9e]
  /checkout/obj/build/x86_64-unknown-linux-gnu/stage1/lib/librustc_driver-9f03b5a4bd4fd338.so(+0x13bdeff)[0x7ff4b913deff]
  /checkout/obj/build/x86_64-unknown-linux-gnu/stage1/lib/librustc_driver-9f03b5a4bd4fd338.so(+0x13bd405)[0x7ff4b913d405]
  /checkout/obj/build/x86_64-unknown-linux-gnu/stage1/lib/librustc_driver-9f03b5a4bd4fd338.so(_RNvNtCskZy4SX28Zi5_15rustc_interface9interface15parse_check_cfg+0x97)[0x7ff4b9133db7]
  /checkout/obj/build/x86_64-unknown-linux-gnu/stage1/lib/librustc_driver-9f03b5a4bd4fd338.so(_RNvMs_CseFSIcjScAnI_17rustc_driver_implNtB4_11RunCompiler3run+0x2c1)[0x7ff4b8fc4dc1]
  /checkout/obj/build/x86_64-unknown-linux-gnu/stage1/lib/librustc_driver-9f03b5a4bd4fd338.so(+0x12a1dfb)[0x7ff4b9021dfb]
  /checkout/obj/build/x86_64-unknown-linux-gnu/stage1/lib/librustc_driver-9f03b5a4bd4fd338.so(_RNvCseFSIcjScAnI_17rustc_driver_impl4main+0x167)[0x7ff4b8fcf207]
  /checkout/obj/build/x86_64-unknown-linux-gnu/stage1/bin/rustc(+0x1aa77)[0x55c21dcb6a77]
  /checkout/obj/build/x86_64-unknown-linux-gnu/stage1/bin/rustc(+0x1aad3)[0x55c21dcb6ad3]
  /checkout/obj/build/x86_64-unknown-linux-gnu/stage1/bin/rustc(+0x1aac9)[0x55c21dcb6ac9]
  /checkout/obj/build/x86_64-unknown-linux-gnu/stage1/lib/libstd-5edd1f0c45987863.so(_ZN3std2rt19lang_start_internal17ha6231f3553cf162cE+0x42c)[0x7ff4bbeb81cc]
  /checkout/obj/build/x86_64-unknown-linux-gnu/stage1/bin/rustc(+0x1aaa8)[0x55c21dcb6aa8]
  /lib64/libc.so.6(__libc_start_main+0xf5)[0x7ff4b73ac555]
  /checkout/obj/build/x86_64-unknown-linux-gnu/stage1/bin/rustc(+0x1a989)[0x55c21dcb6989]
  [RUSTC-TIMING] ___ test:false 0.315
  rustc exited with signal: 11 (SIGSEGV) (core dumped)
stage-build INFO: Section `Stage 1 (LLVM PGO) > Build rustc and LLVM` ended: FAIL (625.47s)
stage-build INFO: Section `Stage 1 (LLVM PGO)` ended: FAIL (625.47s)
stage-build ERROR: The multi-stage build has failed
stage-build INFO: Timer results
---
Total duration:                          10m 25s
------------------------------------------------
root INFO: Free disk space: 594.39 GiB out of total 666.61 GiB (10.83% used)
Traceback (most recent call last):
  File "../src/ci/stage-build.py", line 839, in <module>
    raise e
  File "../src/ci/stage-build.py", line 836, in <module>
    execute_build_pipeline(timer, pipeline, build_args)
  File "../src/ci/stage-build.py", line 760, in execute_build_pipeline
    LLVM_PROFILE_DIR=str(pipeline.llvm_profile_dir_root() / "prof-%p")
  File "../src/ci/stage-build.py", line 571, in build_rustc
    cmd(arguments, env=env)
  File "../src/ci/stage-build.py", line 452, in cmd
    return subprocess.run(args, env=environment, check=True)
  File "/usr/lib64/python3.6/subprocess.py", line 438, in run
    output=stdout, stderr=stderr)
subprocess.CalledProcessError: Command '['/usr/bin/python3', '/checkout/x.py', 'build', '--target', 'x86_64-unknown-linux-gnu', '--host', 'x86_64-unknown-linux-gnu', '--stage', '2', 'library/std', '--llvm-profile-generate']' returned non-zero exit status 1.
