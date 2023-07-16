plain
   Compiling rustc_trait_selection v0.0.0 (/checkout/compiler/rustc_trait_selection)
error[E0425]: cannot find value `infcx` in this scope
   --> compiler/rustc_trait_selection/src/traits/select/mod.rs:540:24
    |
540 |             let goal = infcx.resolve_vars_if_possible((obligation.predicate, obligation.param_env));
    |                        ^^^^^ help: you might have meant to use the available field: `self.infcx`
For more information about this error, try `rustc --explain E0425`.
[RUSTC-TIMING] rustc_trait_selection test:false 3.338
error: could not compile `rustc_trait_selection` due to previous error
warning: build failed, waiting for other jobs to finish...
---
Total duration:                            9m 5s
------------------------------------------------
root INFO: Free disk space: 509.58 GiB out of total 581.32 GiB (12.34% used)
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
