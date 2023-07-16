plain
   Compiling rustc_lint v0.0.0 (/checkout/compiler/rustc_lint)
error[E0425]: cannot find value `suggestion` in module `crate::fluent_generated`
  --> compiler/rustc_lint/src/lints.rs:64:24
   |
64 | #[multipart_suggestion(suggestion, applicability = "machine-applicable")]
   |                        ^^^^^^^^^^ not found in `crate::fluent_generated`
help: consider importing one of these items
   |
3  | use crate::fluent_generated::_subdiag::suggestion;
   |
---
Total duration:                           8m 36s
------------------------------------------------
root INFO: Free disk space: 595.14 GiB out of total 666.61 GiB (10.72% used)
Traceback (most recent call last):
  File "../src/ci/stage-build.py", line 837, in <module>
    raise e
  File "../src/ci/stage-build.py", line 834, in <module>
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
