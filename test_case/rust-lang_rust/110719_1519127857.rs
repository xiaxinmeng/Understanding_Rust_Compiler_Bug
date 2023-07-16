plain
[RUSTC-TIMING] build_script_build test:false 0.353
error: failed to run custom build command for `log v0.4.14`

Caused by:
  process didn't exit successfully: `/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/release/build/log-d0b86ba3a4fe8bce/build-script-build` (signal: 11, SIGSEGV: invalid memory reference)
error: failed to run custom build command for `memchr v2.5.0`

Caused by:
Caused by:
  process didn't exit successfully: `/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/release/build/memchr-687023b910f052dc/build-script-build` (signal: 11, SIGSEGV: invalid memory reference)
error: failed to run custom build command for `parking_lot_core v0.8.5`
Caused by:
Caused by:
  process didn't exit successfully: `/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/release/build/parking_lot_core-976157e0c03b16e0/build-script-build` (signal: 11, SIGSEGV: invalid memory reference)

Caused by:
Caused by:
  process didn't exit successfully: `/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/release/build/quote-16fe8f1c6301bd28/build-script-build` (signal: 11, SIGSEGV: invalid memory reference)
error: failed to run custom build command for `syn v1.0.102`

Caused by:
Caused by:
  process didn't exit successfully: `/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/release/build/syn-95c67889968b8de5/build-script-build` (signal: 11, SIGSEGV: invalid memory reference)
error: failed to run custom build command for `libc v0.2.140`

Caused by:
Caused by:
  process didn't exit successfully: `/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/release/build/libc-80e090524687cd7f/build-script-build` (signal: 11, SIGSEGV: invalid memory reference)

Caused by:
Caused by:
  process didn't exit successfully: `/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/release/build/proc-macro2-cf90d4a2896730d7/build-script-build` (signal: 11, SIGSEGV: invalid memory reference)

Caused by:
Caused by:
  process didn't exit successfully: `/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/release/build/proc-macro-hack-ed19a6077bc1b727/build-script-build` (signal: 11, SIGSEGV: invalid memory reference)
[RUSTC-TIMING] thin_vec test:false 0.432
[RUSTC-TIMING] build_script_main test:false 0.485
[RUSTC-TIMING] tracing_core test:false 0.863
[RUSTC-TIMING] cc test:false 1.110
---
Total duration:                          10m 51s
------------------------------------------------
root INFO: Free disk space: 509.12 GiB out of total 581.32 GiB (12.42% used)
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
