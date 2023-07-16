plain
   Compiling rustc_mir_transform v0.0.0 (/checkout/compiler/rustc_mir_transform)
   Compiling rustc_borrowck v0.0.0 (/checkout/compiler/rustc_borrowck)
[RUSTC-TIMING] rustc_middle test:false 58.754
   Compiling rustc_plugin_impl v0.0.0 (/checkout/compiler/rustc_plugin_impl)
error: variants `AfterConstProp` and `Final` are never constructed
 --> compiler/rustc_mir_transform/src/simplify_branches.rs:6:5
5 | pub enum SimplifyConstCondition {
  |          ---------------------- variants in this enum
6 |     AfterConstProp,
  |     ^^^^^^^^^^^^^^
  |     ^^^^^^^^^^^^^^
7 |     Final,
  |     ^^^^^
  |
  = note: `-D dead-code` implied by `-D warnings`
[RUSTC-TIMING] rustc_mir_transform test:false 3.482
error: could not compile `rustc_mir_transform` (lib) due to previous error
warning: build failed, waiting for other jobs to finish...
[RUSTC-TIMING] rustc_privacy test:false 8.421
---
Total duration:                           9m 15s
------------------------------------------------
root INFO: Free disk space: 511.29 GiB out of total 581.32 GiB (12.04% used)
Traceback (most recent call last):
  File "../src/ci/stage-build.py", line 870, in <module>
    run(runner)
  File "../src/ci/stage-build.py", line 861, in run
    raise e
  File "../src/ci/stage-build.py", line 858, in run
    execute_build_pipeline(timer, pipeline, runner, build_args)
  File "../src/ci/stage-build.py", line 781, in execute_build_pipeline
    LLVM_PROFILE_DIR=str(pipeline.llvm_profile_dir_root() / "prof-%p")
  File "../src/ci/stage-build.py", line 608, in build_rustc
    cmd(arguments, env=env)
  File "../src/ci/stage-build.py", line 451, in cmd
    return subprocess.run(args, env=environment, check=True)
  File "/usr/lib64/python3.6/subprocess.py", line 438, in run
    output=stdout, stderr=stderr)
subprocess.CalledProcessError: Command '['/usr/bin/python3', '/checkout/x.py', 'build', '--target', 'x86_64-unknown-linux-gnu', '--host', 'x86_64-unknown-linux-gnu', '--stage', '2', 'library/std', '--llvm-profile-generate']' returned non-zero exit status 1.
