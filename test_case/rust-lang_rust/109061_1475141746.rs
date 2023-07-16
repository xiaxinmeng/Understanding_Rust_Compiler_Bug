plain
   Compiling rustc_traits v0.0.0 (/checkout/compiler/rustc_traits)
error: unused import: `symbol::sym`
  --> compiler/rustc_const_eval/src/interpret/memory.rs:19:18
   |
19 | use rustc_span::{symbol::sym, FileNameDisplayPreference};
   |
   |
   = note: `-D unused-imports` implied by `-D warnings`
[RUSTC-TIMING] rustc_const_eval test:false 3.623
error: could not compile `rustc_const_eval` due to previous error
warning: build failed, waiting for other jobs to finish...
[RUSTC-TIMING] rustc_ast_lowering test:false 21.552
---
Total duration:                          10m 25s
------------------------------------------------
root INFO: Free disk space: 594.81 GiB out of total 666.61 GiB (10.77% used)
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
