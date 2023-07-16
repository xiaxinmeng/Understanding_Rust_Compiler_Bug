plain
Building stage1 library artifacts (x86_64-unknown-linux-gnu) 
error: failed to run `rustc` to learn about target-specific information

Caused by:
  process didn't exit successfully: `/checkout/obj/build/bootstrap/debug/rustc - --crate-name ___ --print=file-names -Csymbol-mangling-version=legacy -Zunstable-options -Zunstable-options '--check-cfg=values(bootstrap)' '--check-cfg=values(stdarch_intel_sde)' '--check-cfg=values(no_fp_fmt_parse)' '--check-cfg=values(no_global_oom_handling)' '--check-cfg=values(no_rc)' '--check-cfg=values(no_sync)' '--check-cfg=values(freebsd12)' '--check-cfg=values(freebsd13)' '--check-cfg=values(backtrace_in_libstd)' '--check-cfg=values(target_env,"libnx")' '--check-cfg=values(target_arch,"asmjs","spirv","nvptx","xtensa")' -Zmacro-backtrace -Clink-args=-Wl,-z,origin '-Clink-args=-Wl,-rpath,$ORIGIN/../lib' -Clink-args=-fuse-ld=lld -Csplit-debuginfo=off -Cprefer-dynamic -Zinline-mir -Cembed-bitcode=yes '-Zcrate-attr=doc(html_root_url="https://doc.rust-lang.org/nightly/")' --target x86_64-unknown-linux-gnu --crate-type bin --crate-type rlib --crate-type dylib --crate-type cdylib --crate-type staticlib --crate-type proc-macro --print=sysroot --print=split-debuginfo --print=crate-name --print=cfg` (exit status: 254)
  --- stderr
  /checkout/obj/build/x86_64-unknown-linux-gnu/stage1/lib/librustc_driver-d8b5aaf433c6b018.so(+0x125d583)[0x7fb817915583]
  /lib64/libpthread.so.0(+0xf630)[0x7fb81609f630]
  /checkout/obj/build/x86_64-unknown-linux-gnu/stage1/bin/rustc(+0x1c33f)[0x55b01deb133f]
  /checkout/obj/build/x86_64-unknown-linux-gnu/stage1/lib/librustc_driver-d8b5aaf433c6b018.so(+0x12a02ee)[0x7fb8179582ee]
  /checkout/obj/build/x86_64-unknown-linux-gnu/stage1/lib/librustc_driver-d8b5aaf433c6b018.so(+0x129a63f)[0x7fb81795263f]
  /checkout/obj/build/x86_64-unknown-linux-gnu/stage1/lib/librustc_driver-d8b5aaf433c6b018.so(+0x1299b75)[0x7fb817951b75]
  /checkout/obj/build/x86_64-unknown-linux-gnu/stage1/lib/librustc_driver-d8b5aaf433c6b018.so(_RNvNtCsaXHAFpfKoep_15rustc_interface9interface15parse_check_cfg+0x97)[0x7fb817a4c3e7]
  /checkout/obj/build/x86_64-unknown-linux-gnu/stage1/lib/librustc_driver-d8b5aaf433c6b018.so(_RNvMs_Csd2UaMTeiMW3_17rustc_driver_implNtB4_11RunCompiler3run+0x2c1)[0x7fb8178799f1]
  /checkout/obj/build/x86_64-unknown-linux-gnu/stage1/lib/librustc_driver-d8b5aaf433c6b018.so(+0x11e31fb)[0x7fb81789b1fb]
  /checkout/obj/build/x86_64-unknown-linux-gnu/stage1/lib/librustc_driver-d8b5aaf433c6b018.so(_RNvCsd2UaMTeiMW3_17rustc_driver_impl4main+0x165)[0x7fb817882235]
  /checkout/obj/build/x86_64-unknown-linux-gnu/stage1/bin/rustc(+0x19d57)[0x55b01deaed57]
  /checkout/obj/build/x86_64-unknown-linux-gnu/stage1/bin/rustc(+0x19db3)[0x55b01deaedb3]
  /checkout/obj/build/x86_64-unknown-linux-gnu/stage1/bin/rustc(+0x19da9)[0x55b01deaeda9]
  /checkout/obj/build/x86_64-unknown-linux-gnu/stage1/lib/libstd-5edd1f0c45987863.so(_ZN3std2rt19lang_start_internal17ha6231f3553cf162cE+0x42c)[0x7fb81a7cd34c]
  /checkout/obj/build/x86_64-unknown-linux-gnu/stage1/bin/rustc(+0x19d88)[0x55b01deaed88]
  /lib64/libc.so.6(__libc_start_main+0xf5)[0x7fb815ce4555]
  /checkout/obj/build/x86_64-unknown-linux-gnu/stage1/bin/rustc(+0x19c89)[0x55b01deaec89]
  [RUSTC-TIMING] ___ test:false 0.208
  rustc exited with signal: 11 (SIGSEGV) (core dumped)
stage-build INFO: Section `Stage 1 (LLVM PGO) > Build rustc and LLVM` ended: FAIL (675.92s)
stage-build INFO: Section `Stage 1 (LLVM PGO)` ended: FAIL (675.92s)
stage-build ERROR: The multi-stage build has failed
stage-build INFO: Timer results
---
Total duration:                          11m 16s
------------------------------------------------
root INFO: Free disk space: 508.55 GiB out of total 581.32 GiB (12.51% used)
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
