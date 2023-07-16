plain
   Compiling rustc_ast_passes v0.0.0 (/checkout/compiler/rustc_ast_passes)
error: unused import: `CrateStoreDyn`
  --> compiler/rustc_middle/src/ty/context.rs:58:29
   |
58 | use rustc_session::cstore::{CrateStoreDyn, Untracked};
   |
   |
   = note: `-D unused-imports` implied by `-D warnings`
   Compiling rustc_expand v0.0.0 (/checkout/compiler/rustc_expand)
[RUSTC-TIMING] rustc_session test:false 5.032
   Compiling rustc_builtin_macros v0.0.0 (/checkout/compiler/rustc_builtin_macros)
[RUSTC-TIMING] rustc_ast_passes test:false 3.742
---
Total duration:                          6m 27s
-----------------------------------------------
root INFO: Free disk space: 598.83 GiB out of total 666.61 GiB (10.16% used)
Traceback (most recent call last):
  File "../src/ci/stage-build.py", line 747, in <module>
    raise e
  File "../src/ci/stage-build.py", line 744, in <module>
    execute_build_pipeline(timer, pipeline, build_args)
  File "../src/ci/stage-build.py", line 675, in execute_build_pipeline
    LLVM_PROFILE_DIR=str(pipeline.llvm_profile_dir_root() / "prof-%p")
  File "../src/ci/stage-build.py", line 524, in build_rustc
    cmd(arguments, env=env)
  File "../src/ci/stage-build.py", line 405, in cmd
    return subprocess.run(args, env=environment, check=True)
  File "/usr/lib64/python3.6/subprocess.py", line 438, in run
    output=stdout, stderr=stderr)
subprocess.CalledProcessError: Command '['/usr/bin/python3', '/checkout/x.py', 'build', '--target', 'x86_64-unknown-linux-gnu', '--host', 'x86_64-unknown-linux-gnu', '--stage', '2', 'library/std', '--llvm-profile-generate']' returned non-zero exit status 1.
