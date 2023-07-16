plain
   |
22 | const BLOCK_LIMIT: usize = 100;
   |       ^^^^^^^^^^^
   |
   = note: `-D dead-code` implied by `-D warnings`
[RUSTC-TIMING] rustc_mir_transform test:false 2.690
error: could not compile `rustc_mir_transform` due to previous error
warning: build failed, waiting for other jobs to finish...
[RUSTC-TIMING] rustc_codegen_ssa test:false 39.982
---
Total duration:                           9m 18s
------------------------------------------------
root INFO: Free disk space: 510.89 GiB out of total 581.32 GiB (12.11% used)
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
