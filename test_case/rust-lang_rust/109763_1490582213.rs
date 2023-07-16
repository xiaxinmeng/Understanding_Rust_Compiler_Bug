plain
   Compiling rustc_mir_transform v0.0.0 (/checkout/compiler/rustc_mir_transform)
error[E0308]: mismatched types
   --> compiler/rustc_traits/src/implied_outlives_bounds.rs:275:59
    |
275 |             Component::UnresolvedInferenceVariable(..) => {}
    |                                                           ^^ expected `Option<OutlivesBound<'_>>`, found `()`
    |
    = note:   expected enum `std::option::Option<OutlivesBound<'_>>`

For more information about this error, try `rustc --explain E0308`.
[RUSTC-TIMING] rustc_traits test:false 0.447
error: could not compile `rustc_traits` due to previous error
---
Total duration:                           9m 20s
------------------------------------------------
root INFO: Free disk space: 509.86 GiB out of total 581.32 GiB (12.29% used)
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
