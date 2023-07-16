plain
Building stage1 library artifacts (x86_64-unknown-linux-gnu) 
error: failed to run `rustc` to learn about target-specific information

Caused by:
  process didn't exit successfully: `/checkout/obj/build/bootstrap/debug/rustc - --crate-name ___ --print=file-names -Csymbol-mangling-version=legacy -Zunstable-options -Zunstable-options '--check-cfg=values(bootstrap)' '--check-cfg=values(stdarch_intel_sde)' '--check-cfg=values(no_fp_fmt_parse)' '--check-cfg=values(no_global_oom_handling)' '--check-cfg=values(no_rc)' '--check-cfg=values(no_sync)' '--check-cfg=values(freebsd12)' '--check-cfg=values(backtrace_in_libstd)' '--check-cfg=values(target_env,"libnx")' '--check-cfg=values(target_arch,"asmjs","spirv","nvptx","xtensa")' -Zmacro-backtrace -Clink-args=-Wl,-z,origin '-Clink-args=-Wl,-rpath,$ORIGIN/../lib' -Clink-args=-fuse-ld=lld -Csplit-debuginfo=off -Cprefer-dynamic -Cembed-bitcode=yes '-Zcrate-attr=doc(html_root_url="https://doc.rust-lang.org/nightly/")' --target x86_64-unknown-linux-gnu --crate-type bin --crate-type rlib --crate-type dylib --crate-type cdylib --crate-type staticlib --crate-type proc-macro --print=sysroot --print=split-debuginfo --print=crate-name --print=cfg` (exit status: 254)
  --- stderr
  /checkout/obj/build/x86_64-unknown-linux-gnu/stage1/lib/librustc_driver-9f03b5a4bd4fd338.so(+0x1264783)[0x7f7aaeb5b783]
  /lib64/libpthread.so.0(+0xf630)[0x7f7aad2de630]
  /checkout/obj/build/x86_64-unknown-linux-gnu/stage1/bin/rustc(+0x1c35f)[0x55885bb1735f]
  /checkout/obj/build/x86_64-unknown-linux-gnu/stage1/lib/librustc_driver-9f03b5a4bd4fd338.so(+0x1371bae)[0x7f7aaec68bae]
  /checkout/obj/build/x86_64-unknown-linux-gnu/stage1/lib/librustc_driver-9f03b5a4bd4fd338.so(+0x136fc0f)[0x7f7aaec66c0f]
  /checkout/obj/build/x86_64-unknown-linux-gnu/stage1/lib/librustc_driver-9f03b5a4bd4fd338.so(_RNvNtCskZy4SX28Zi5_15rustc_interface9interface15parse_check_cfg+0xc5)[0x7f7aaec70c85]
  /checkout/obj/build/x86_64-unknown-linux-gnu/stage1/lib/librustc_driver-9f03b5a4bd4fd338.so(_RNvMs_CseFSIcjScAnI_17rustc_driver_implNtB4_11RunCompiler3run+0x2c1)[0x7f7aaeae2e21]
  /checkout/obj/build/x86_64-unknown-linux-gnu/stage1/lib/librustc_driver-9f03b5a4bd4fd338.so(+0x1263dbb)[0x7f7aaeb5adbb]
  /checkout/obj/build/x86_64-unknown-linux-gnu/stage1/lib/librustc_driver-9f03b5a4bd4fd338.so(_RNvCseFSIcjScAnI_17rustc_driver_impl4main+0x167)[0x7f7aaeaeb657]
  /checkout/obj/build/x86_64-unknown-linux-gnu/stage1/bin/rustc(+0x19d77)[0x55885bb14d77]
  /checkout/obj/build/x86_64-unknown-linux-gnu/stage1/bin/rustc(+0x19dd3)[0x55885bb14dd3]
  /checkout/obj/build/x86_64-unknown-linux-gnu/stage1/bin/rustc(+0x19dc9)[0x55885bb14dc9]
  /checkout/obj/build/x86_64-unknown-linux-gnu/stage1/lib/libstd-5edd1f0c45987863.so(_ZN3std2rt19lang_start_internal17ha6231f3553cf162cE+0x42c)[0x7f7ab1a20fbc]
  /checkout/obj/build/x86_64-unknown-linux-gnu/stage1/bin/rustc(+0x19da8)[0x55885bb14da8]
  /lib64/libc.so.6(__libc_start_main+0xf5)[0x7f7aacf23555]
  /checkout/obj/build/x86_64-unknown-linux-gnu/stage1/bin/rustc(+0x19c89)[0x55885bb14c89]
  [RUSTC-TIMING] ___ test:false 0.207
  rustc exited with signal: 11 (SIGSEGV) (core dumped)
stage-build INFO: Section `Stage 1 (LLVM PGO) > Build rustc and LLVM` ended: FAIL (585.79s)
stage-build INFO: Section `Stage 1 (LLVM PGO)` ended: FAIL (585.79s)
stage-build ERROR: The multi-stage build has failed
stage-build INFO: Timer results
---
Total duration:                           9m 46s
------------------------------------------------
root INFO: Free disk space: 509.70 GiB out of total 581.32 GiB (12.32% used)
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
