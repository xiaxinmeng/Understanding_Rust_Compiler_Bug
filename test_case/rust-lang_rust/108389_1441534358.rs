plain
   Compiling rustc_monomorphize v0.0.0 (/checkout/compiler/rustc_monomorphize)
error[E0308]: mismatched types
   --> compiler/rustc_infer/src/infer/canonical/query_response.rs:506:71
    |
506 |                 .extend(self.at(cause, param_env).define_opaque_types(true).eq(a, b)?.obligations);
    |                                                   ------------------- ^^^^ expected enum `DefiningAnchor`, found `bool`
    |                                                   arguments to this method are incorrect
    |
note: associated function defined here
   --> compiler/rustc_infer/src/infer/at.rs:92:12
   --> compiler/rustc_infer/src/infer/at.rs:92:12
    |
92  |     pub fn define_opaque_types(self, define_opaque_types: DefiningAnchor) -> Self {

For more information about this error, try `rustc --explain E0308`.
[RUSTC-TIMING] rustc_infer test:false 2.485
error: could not compile `rustc_infer` due to previous error
---
Total duration:                           8m 40s
------------------------------------------------
root INFO: Free disk space: 595.27 GiB out of total 666.61 GiB (10.70% used)
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
