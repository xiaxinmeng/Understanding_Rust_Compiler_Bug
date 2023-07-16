plain
    |
note: the function `utf8_char_width` is defined here
   --> library/core/src/str/validations.rs:373:1
    |
373 | const fn utf8_char_width(byte: u8) -> usize {

error[E0603]: function `utf8_char_width` is private
   --> library/core/src/str/mod.rs:74:40
    |
    |
74  | pub use validations::{next_code_point, utf8_char_width};
    |
note: the function `utf8_char_width` is defined here
   --> library/core/src/str/validations.rs:373:1
    |
    |
373 | const fn utf8_char_width(byte: u8) -> usize {

For more information about this error, try `rustc --explain E0603`.
[RUSTC-TIMING] core test:false 4.711
error: could not compile `core` (lib) due to 2 previous errors
---
Total duration:                              12s
------------------------------------------------
root INFO: Free disk space: 516.36 GiB out of total 581.32 GiB (11.17% used)
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
